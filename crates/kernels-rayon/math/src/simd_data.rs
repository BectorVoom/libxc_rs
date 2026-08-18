//! Data tables and constants for the glibc-bit-identical SIMD `exp`/`ln`.
//!
//! Transcribed from ARM optimized-routines (`math/exp_data.c`,
//! `math/log_data.c`, N = 128 variants), Copyright (c) 2018-2023 Arm
//! Limited, SPDX-License-Identifier: MIT OR Apache-2.0 WITH LLVM-exception.
//! These are byte-identical to the tables glibc 2.43 compiles into
//! `__ieee754_exp_fma` / `__ieee754_log_fma` (checked against
//! `sysdeps/ieee754/dbl-64/e_{exp,log}_data.c`), which is what makes the
//! SIMD forms bit-identical to the scalar `f64::exp` / `f64::ln` calls the
//! scalar kernels make. Do not edit by hand; values are exact bit patterns.

/// N/ln2, N = 128 (`invln2N`).
pub const EXP_INVLN2N: f64 = f64::from_bits(0x40671547652b82fe); // 0x1.71547652b82fep+7
pub const EXP_NEGLN2HIN: f64 = f64::from_bits(0xbf762e42fefa0000); // -0x1.62e42fefa0000p-8
pub const EXP_NEGLN2LON: f64 = f64::from_bits(0xbd0cf79abc9e3b3a); // -0x1.cf79abc9e3b3ap-47
pub const EXP_SHIFT: f64 = f64::from_bits(0x4338000000000000); // 0x1.8000000000000p+52
pub const EXP_C2: f64 = f64::from_bits(0x3fdffffffffffdbd); // 0x1.ffffffffffdbdp-2
pub const EXP_C3: f64 = f64::from_bits(0x3fc555555555543c); // 0x1.555555555543cp-3
pub const EXP_C4: f64 = f64::from_bits(0x3fa55555cf172b91); // 0x1.55555cf172b91p-5
pub const EXP_C5: f64 = f64::from_bits(0x3f81111167a4d017); // 0x1.1111167a4d017p-7

/// `2^(k/128) ~= from_bits(TAB[2k+1] + (k<<45)) * (1 + from_bits(TAB[2k]))`.
pub static EXP_TAB: [u64; 256] = [
    0x0000000000000000, 0x3ff0000000000000, 0x3c9b3b4f1a88bf6e, 0x3feff63da9fb3335,
    0xbc7160139cd8dc5d, 0x3fefec9a3e778061, 0xbc905e7a108766d1, 0x3fefe315e86e7f85,
    0x3c8cd2523567f613, 0x3fefd9b0d3158574, 0xbc8bce8023f98efa, 0x3fefd06b29ddf6de,
    0x3c60f74e61e6c861, 0x3fefc74518759bc8, 0x3c90a3e45b33d399, 0x3fefbe3ecac6f383,
    0x3c979aa65d837b6d, 0x3fefb5586cf9890f, 0x3c8eb51a92fdeffc, 0x3fefac922b7247f7,
    0x3c3ebe3d702f9cd1, 0x3fefa3ec32d3d1a2, 0xbc6a033489906e0b, 0x3fef9b66affed31b,
    0xbc9556522a2fbd0e, 0x3fef9301d0125b51, 0xbc5080ef8c4eea55, 0x3fef8abdc06c31cc,
    0xbc91c923b9d5f416, 0x3fef829aaea92de0, 0x3c80d3e3e95c55af, 0x3fef7a98c8a58e51,
    0xbc801b15eaa59348, 0x3fef72b83c7d517b, 0xbc8f1ff055de323d, 0x3fef6af9388c8dea,
    0x3c8b898c3f1353bf, 0x3fef635beb6fcb75, 0xbc96d99c7611eb26, 0x3fef5be084045cd4,
    0x3c9aecf73e3a2f60, 0x3fef54873168b9aa, 0xbc8fe782cb86389d, 0x3fef4d5022fcd91d,
    0x3c8a6f4144a6c38d, 0x3fef463b88628cd6, 0x3c807a05b0e4047d, 0x3fef3f49917ddc96,
    0x3c968efde3a8a894, 0x3fef387a6e756238, 0x3c875e18f274487d, 0x3fef31ce4fb2a63f,
    0x3c80472b981fe7f2, 0x3fef2b4565e27cdd, 0xbc96b87b3f71085e, 0x3fef24dfe1f56381,
    0x3c82f7e16d09ab31, 0x3fef1e9df51fdee1, 0xbc3d219b1a6fbffa, 0x3fef187fd0dad990,
    0x3c8b3782720c0ab4, 0x3fef1285a6e4030b, 0x3c6e149289cecb8f, 0x3fef0cafa93e2f56,
    0x3c834d754db0abb6, 0x3fef06fe0a31b715, 0x3c864201e2ac744c, 0x3fef0170fc4cd831,
    0x3c8fdd395dd3f84a, 0x3feefc08b26416ff, 0xbc86a3803b8e5b04, 0x3feef6c55f929ff1,
    0xbc924aedcc4b5068, 0x3feef1a7373aa9cb, 0xbc9907f81b512d8e, 0x3feeecae6d05d866,
    0xbc71d1e83e9436d2, 0x3feee7db34e59ff7, 0xbc991919b3ce1b15, 0x3feee32dc313a8e5,
    0x3c859f48a72a4c6d, 0x3feedea64c123422, 0xbc9312607a28698a, 0x3feeda4504ac801c,
    0xbc58a78f4817895b, 0x3feed60a21f72e2a, 0xbc7c2c9b67499a1b, 0x3feed1f5d950a897,
    0x3c4363ed60c2ac11, 0x3feece086061892d, 0x3c9666093b0664ef, 0x3feeca41ed1d0057,
    0x3c6ecce1daa10379, 0x3feec6a2b5c13cd0, 0x3c93ff8e3f0f1230, 0x3feec32af0d7d3de,
    0x3c7690cebb7aafb0, 0x3feebfdad5362a27, 0x3c931dbdeb54e077, 0x3feebcb299fddd0d,
    0xbc8f94340071a38e, 0x3feeb9b2769d2ca7, 0xbc87deccdc93a349, 0x3feeb6daa2cf6642,
    0xbc78dec6bd0f385f, 0x3feeb42b569d4f82, 0xbc861246ec7b5cf6, 0x3feeb1a4ca5d920f,
    0x3c93350518fdd78e, 0x3feeaf4736b527da, 0x3c7b98b72f8a9b05, 0x3feead12d497c7fd,
    0x3c9063e1e21c5409, 0x3feeab07dd485429, 0x3c34c7855019c6ea, 0x3feea9268a5946b7,
    0x3c9432e62b64c035, 0x3feea76f15ad2148, 0xbc8ce44a6199769f, 0x3feea5e1b976dc09,
    0xbc8c33c53bef4da8, 0x3feea47eb03a5585, 0xbc845378892be9ae, 0x3feea34634ccc320,
    0xbc93cedd78565858, 0x3feea23882552225, 0x3c5710aa807e1964, 0x3feea155d44ca973,
    0xbc93b3efbf5e2228, 0x3feea09e667f3bcd, 0xbc6a12ad8734b982, 0x3feea012750bdabf,
    0xbc6367efb86da9ee, 0x3fee9fb23c651a2f, 0xbc80dc3d54e08851, 0x3fee9f7df9519484,
    0xbc781f647e5a3ecf, 0x3fee9f75e8ec5f74, 0xbc86ee4ac08b7db0, 0x3fee9f9a48a58174,
    0xbc8619321e55e68a, 0x3fee9feb564267c9, 0x3c909ccb5e09d4d3, 0x3feea0694fde5d3f,
    0xbc7b32dcb94da51d, 0x3feea11473eb0187, 0x3c94ecfd5467c06b, 0x3feea1ed0130c132,
    0x3c65ebe1abd66c55, 0x3feea2f336cf4e62, 0xbc88a1c52fb3cf42, 0x3feea427543e1a12,
    0xbc9369b6f13b3734, 0x3feea589994cce13, 0xbc805e843a19ff1e, 0x3feea71a4623c7ad,
    0xbc94d450d872576e, 0x3feea8d99b4492ed, 0x3c90ad675b0e8a00, 0x3feeaac7d98a6699,
    0x3c8db72fc1f0eab4, 0x3feeace5422aa0db, 0xbc65b6609cc5e7ff, 0x3feeaf3216b5448c,
    0x3c7bf68359f35f44, 0x3feeb1ae99157736, 0xbc93091fa71e3d83, 0x3feeb45b0b91ffc6,
    0xbc5da9b88b6c1e29, 0x3feeb737b0cdc5e5, 0xbc6c23f97c90b959, 0x3feeba44cbc8520f,
    0xbc92434322f4f9aa, 0x3feebd829fde4e50, 0xbc85ca6cd7668e4b, 0x3feec0f170ca07ba,
    0x3c71affc2b91ce27, 0x3feec49182a3f090, 0x3c6dd235e10a73bb, 0x3feec86319e32323,
    0xbc87c50422622263, 0x3feecc667b5de565, 0x3c8b1c86e3e231d5, 0x3feed09bec4a2d33,
    0xbc91bbd1d3bcbb15, 0x3feed503b23e255d, 0x3c90cc319cee31d2, 0x3feed99e1330b358,
    0x3c8469846e735ab3, 0x3feede6b5579fdbf, 0xbc82dfcd978e9db4, 0x3feee36bbfd3f37a,
    0x3c8c1a7792cb3387, 0x3feee89f995ad3ad, 0xbc907b8f4ad1d9fa, 0x3feeee07298db666,
    0xbc55c3d956dcaeba, 0x3feef3a2b84f15fb, 0xbc90a40e3da6f640, 0x3feef9728de5593a,
    0xbc68d6f438ad9334, 0x3feeff76f2fb5e47, 0xbc91eee26b588a35, 0x3fef05b030a1064a,
    0x3c74ffd70a5fddcd, 0x3fef0c1e904bc1d2, 0xbc91bdfbfa9298ac, 0x3fef12c25bd71e09,
    0x3c736eae30af0cb3, 0x3fef199bdd85529c, 0x3c8ee3325c9ffd94, 0x3fef20ab5fffd07a,
    0x3c84e08fd10959ac, 0x3fef27f12e57d14b, 0x3c63cdaf384e1a67, 0x3fef2f6d9406e7b5,
    0x3c676b2c6c921968, 0x3fef3720dcef9069, 0xbc808a1883ccb5d2, 0x3fef3f0b555dc3fa,
    0xbc8fad5d3ffffa6f, 0x3fef472d4a07897c, 0xbc900dae3875a949, 0x3fef4f87080d89f2,
    0x3c74a385a63d07a7, 0x3fef5818dcfba487, 0xbc82919e2040220f, 0x3fef60e316c98398,
    0x3c8e5a50d5c192ac, 0x3fef69e603db3285, 0x3c843a59ac016b4b, 0x3fef7321f301b460,
    0xbc82d52107b43e1f, 0x3fef7c97337b9b5f, 0xbc892ab93b470dc9, 0x3fef864614f5a129,
    0x3c74b604603a88d3, 0x3fef902ee78b3ff6, 0x3c83c5ec519d7271, 0x3fef9a51fbc74c83,
    0xbc8ff7128fd391f0, 0x3fefa4afa2a490da, 0xbc8dae98e223747d, 0x3fefaf482d8e67f1,
    0x3c8ec3bc41aa2008, 0x3fefba1bee615a27, 0x3c842b94c3a9eb32, 0x3fefc52b376bba97,
    0x3c8a64a931d185ee, 0x3fefd0765b6e4540, 0xbc8e37bae43be3ed, 0x3fefdbfdad9cbe14,
    0x3c77893b4d91cd9d, 0x3fefe7c1819e90d8, 0x3c5305c14160cc89, 0x3feff3c22b8f71f1,
];

pub const LOG_LN2HI: f64 = f64::from_bits(0x3fe62e42fefa3800); // 0x1.62e42fefa3800p-1
pub const LOG_LN2LO: f64 = f64::from_bits(0x3d2ef35793c76730); // 0x1.ef35793c76730p-45
pub const LOG_A0: f64 = f64::from_bits(0xbfe0000000000001); // -0x1.0000000000001p-1
pub const LOG_A1: f64 = f64::from_bits(0x3fd555555551305b); // 0x1.555555551305bp-2
pub const LOG_A2: f64 = f64::from_bits(0xbfcfffffffeb4590); // -0x1.fffffffeb4590p-3
pub const LOG_A3: f64 = f64::from_bits(0x3fc999b324f10111); // 0x1.999b324f10111p-3
pub const LOG_A4: f64 = f64::from_bits(0xbfc55575e506c89f); // -0x1.55575e506c89fp-3
pub const LOG_B0: f64 = f64::from_bits(0xbfe0000000000000); // -0x1.0000000000000p-1
pub const LOG_B1: f64 = f64::from_bits(0x3fd5555555555577); // 0x1.5555555555577p-2
pub const LOG_B2: f64 = f64::from_bits(0xbfcffffffffffdcb); // -0x1.ffffffffffdcbp-3
pub const LOG_B3: f64 = f64::from_bits(0x3fc999999995dd0c); // 0x1.999999995dd0cp-3
pub const LOG_B4: f64 = f64::from_bits(0xbfc55555556745a7); // -0x1.55555556745a7p-3
pub const LOG_B5: f64 = f64::from_bits(0x3fc24924a344de30); // 0x1.24924a344de30p-3
pub const LOG_B6: f64 = f64::from_bits(0xbfbfffffa4423d65); // -0x1.fffffa4423d65p-4
pub const LOG_B7: f64 = f64::from_bits(0x3fbc7184282ad6ca); // 0x1.c7184282ad6cap-4
pub const LOG_B8: f64 = f64::from_bits(0xbfb999eb43b068ff); // -0x1.999eb43b068ffp-4
pub const LOG_B9: f64 = f64::from_bits(0x3fb78182f7afd085); // 0x1.78182f7afd085p-4
pub const LOG_B10: f64 = f64::from_bits(0xbfb5521375d145cd); // -0x1.5521375d145cdp-4

/// `[invc, logc]` interleaved, 128 subintervals (`__log_data.tab`).
pub static LOG_TAB: [f64; 256] = [
    f64::from_bits(0x3ff734f0c3e0de9f), f64::from_bits(0xbfd7cc7f79e69000), // 0
    f64::from_bits(0x3ff713786a2ce91f), f64::from_bits(0xbfd76feec20d0000), // 1
    f64::from_bits(0x3ff6f26008fab5a0), f64::from_bits(0xbfd713e31351e000), // 2
    f64::from_bits(0x3ff6d1a61f138c7d), f64::from_bits(0xbfd6b85b38287800), // 3
    f64::from_bits(0x3ff6b1490bc5b4d1), f64::from_bits(0xbfd65d5590807800), // 4
    f64::from_bits(0x3ff69147332f0cba), f64::from_bits(0xbfd602d076180000), // 5
    f64::from_bits(0x3ff6719f18224223), f64::from_bits(0xbfd5a8ca86909000), // 6
    f64::from_bits(0x3ff6524f99a51ed9), f64::from_bits(0xbfd54f4356035000), // 7
    f64::from_bits(0x3ff63356aa8f24c4), f64::from_bits(0xbfd4f637c36b4000), // 8
    f64::from_bits(0x3ff614b36b9ddc14), f64::from_bits(0xbfd49da7fda85000), // 9
    f64::from_bits(0x3ff5f66452c65c4c), f64::from_bits(0xbfd445923989a800), // 10
    f64::from_bits(0x3ff5d867b5912c4f), f64::from_bits(0xbfd3edf439b0b800), // 11
    f64::from_bits(0x3ff5babccb5b90de), f64::from_bits(0xbfd396ce448f7000), // 12
    f64::from_bits(0x3ff59d61f2d91a78), f64::from_bits(0xbfd3401e17bda000), // 13
    f64::from_bits(0x3ff5805612465687), f64::from_bits(0xbfd2e9e2ef468000), // 14
    f64::from_bits(0x3ff56397cee76bd3), f64::from_bits(0xbfd2941b3830e000), // 15
    f64::from_bits(0x3ff54725e2a77f93), f64::from_bits(0xbfd23ec58cda8800), // 16
    f64::from_bits(0x3ff52aff42064583), f64::from_bits(0xbfd1e9e129279000), // 17
    f64::from_bits(0x3ff50f22dbb2bddf), f64::from_bits(0xbfd1956d2b48f800), // 18
    f64::from_bits(0x3ff4f38f4734ded7), f64::from_bits(0xbfd141679ab9f800), // 19
    f64::from_bits(0x3ff4d843cfde2840), f64::from_bits(0xbfd0edd094ef9800), // 20
    f64::from_bits(0x3ff4bd3ec078a3c8), f64::from_bits(0xbfd09aa518db1000), // 21
    f64::from_bits(0x3ff4a27fc3e0258a), f64::from_bits(0xbfd047e65263b800), // 22
    f64::from_bits(0x3ff4880524d48434), f64::from_bits(0xbfcfeb224586f000), // 23
    f64::from_bits(0x3ff46dce1b192d0b), f64::from_bits(0xbfcf474a7517b000), // 24
    f64::from_bits(0x3ff453d9d3391854), f64::from_bits(0xbfcea4443d103000), // 25
    f64::from_bits(0x3ff43a2744b4845a), f64::from_bits(0xbfce020d44e9b000), // 26
    f64::from_bits(0x3ff420b54115f8fb), f64::from_bits(0xbfcd60a22977f000), // 27
    f64::from_bits(0x3ff40782da3ef4b1), f64::from_bits(0xbfccc00104959000), // 28
    f64::from_bits(0x3ff3ee8f5d57fe8f), f64::from_bits(0xbfcc202956891000), // 29
    f64::from_bits(0x3ff3d5d9a00b4ce9), f64::from_bits(0xbfcb81178d811000), // 30
    f64::from_bits(0x3ff3bd60c010c12b), f64::from_bits(0xbfcae2c9ccd3d000), // 31
    f64::from_bits(0x3ff3a5242b75dab8), f64::from_bits(0xbfca45402e129000), // 32
    f64::from_bits(0x3ff38d22cd9fd002), f64::from_bits(0xbfc9a877681df000), // 33
    f64::from_bits(0x3ff3755bc5847a1c), f64::from_bits(0xbfc90c6d69483000), // 34
    f64::from_bits(0x3ff35dce49ad36e2), f64::from_bits(0xbfc87120a645c000), // 35
    f64::from_bits(0x3ff34679984dd440), f64::from_bits(0xbfc7d68fb4143000), // 36
    f64::from_bits(0x3ff32f5cceffcb24), f64::from_bits(0xbfc73cb83c627000), // 37
    f64::from_bits(0x3ff3187775a10d49), f64::from_bits(0xbfc6a39a9b376000), // 38
    f64::from_bits(0x3ff301c8373e3990), f64::from_bits(0xbfc60b3154b7a000), // 39
    f64::from_bits(0x3ff2eb4ebb95f841), f64::from_bits(0xbfc5737d76243000), // 40
    f64::from_bits(0x3ff2d50a0219a9d1), f64::from_bits(0xbfc4dc7b8fc23000), // 41
    f64::from_bits(0x3ff2bef9a8b7fd2a), f64::from_bits(0xbfc4462c51d20000), // 42
    f64::from_bits(0x3ff2a91c7a0c1bab), f64::from_bits(0xbfc3b08abc830000), // 43
    f64::from_bits(0x3ff293726014b530), f64::from_bits(0xbfc31b996b490000), // 44
    f64::from_bits(0x3ff27dfa5757a1f5), f64::from_bits(0xbfc2875490a44000), // 45
    f64::from_bits(0x3ff268b39b1d3bbf), f64::from_bits(0xbfc1f3b9f879a000), // 46
    f64::from_bits(0x3ff2539d838ff5bd), f64::from_bits(0xbfc160c8252ca000), // 47
    f64::from_bits(0x3ff23eb7aac9083b), f64::from_bits(0xbfc0ce7f57f72000), // 48
    f64::from_bits(0x3ff22a012ba940b6), f64::from_bits(0xbfc03cdc49fea000), // 49
    f64::from_bits(0x3ff2157996cc4132), f64::from_bits(0xbfbf57bdbc4b8000), // 50
    f64::from_bits(0x3ff201201dd2fc9b), f64::from_bits(0xbfbe370896404000), // 51
    f64::from_bits(0x3ff1ecf4494d480b), f64::from_bits(0xbfbd17983ef94000), // 52
    f64::from_bits(0x3ff1d8f5528f6569), f64::from_bits(0xbfbbf9674ed8a000), // 53
    f64::from_bits(0x3ff1c52311577e7c), f64::from_bits(0xbfbadc79202f6000), // 54
    f64::from_bits(0x3ff1b17c74cb26e9), f64::from_bits(0xbfb9c0c3e7288000), // 55
    f64::from_bits(0x3ff19e010c2c1ab6), f64::from_bits(0xbfb8a646b372c000), // 56
    f64::from_bits(0x3ff18ab07bb670bd), f64::from_bits(0xbfb78d01b3ac0000), // 57
    f64::from_bits(0x3ff1778a25efbcb6), f64::from_bits(0xbfb674f145380000), // 58
    f64::from_bits(0x3ff1648d354c31da), f64::from_bits(0xbfb55e0e6d878000), // 59
    f64::from_bits(0x3ff151b990275fdd), f64::from_bits(0xbfb4485cdea1e000), // 60
    f64::from_bits(0x3ff13f0ea432d24c), f64::from_bits(0xbfb333d94d6aa000), // 61
    f64::from_bits(0x3ff12c8b7210f9da), f64::from_bits(0xbfb22079f8c56000), // 62
    f64::from_bits(0x3ff11a3028ecb531), f64::from_bits(0xbfb10e4698622000), // 63
    f64::from_bits(0x3ff107fbda8434af), f64::from_bits(0xbfaffa6c6ad20000), // 64
    f64::from_bits(0x3ff0f5ee0f4e6bb3), f64::from_bits(0xbfadda8d4a774000), // 65
    f64::from_bits(0x3ff0e4065d2a9fce), f64::from_bits(0xbfabbcece4850000), // 66
    f64::from_bits(0x3ff0d244632ca521), f64::from_bits(0xbfa9a1894012c000), // 67
    f64::from_bits(0x3ff0c0a77ce2981a), f64::from_bits(0xbfa788583302c000), // 68
    f64::from_bits(0x3ff0af2f83c636d1), f64::from_bits(0xbfa5715e67d68000), // 69
    f64::from_bits(0x3ff09ddb98a01339), f64::from_bits(0xbfa35c8a49658000), // 70
    f64::from_bits(0x3ff08cabaf52e7df), f64::from_bits(0xbfa149e364154000), // 71
    f64::from_bits(0x3ff07b9f2f4e28fb), f64::from_bits(0xbf9e72c082eb8000), // 72
    f64::from_bits(0x3ff06ab58c358f19), f64::from_bits(0xbf9a55f152528000), // 73
    f64::from_bits(0x3ff059eea5ecf92c), f64::from_bits(0xbf963d62cf818000), // 74
    f64::from_bits(0x3ff04949cdd12c90), f64::from_bits(0xbf9228fb8caa0000), // 75
    f64::from_bits(0x3ff038c6c6f0ada9), f64::from_bits(0xbf8c317b20f90000), // 76
    f64::from_bits(0x3ff02865137932a9), f64::from_bits(0xbf8419355daa0000), // 77
    f64::from_bits(0x3ff0182427ea7348), f64::from_bits(0xbf781203c2ec0000), // 78
    f64::from_bits(0x3ff008040614b195), f64::from_bits(0xbf60040979240000), // 79
    f64::from_bits(0x3fefe01ff726fa1a), f64::from_bits(0x3f6feff384900000), // 80
    f64::from_bits(0x3fefa11cc261ea74), f64::from_bits(0x3f87dc41353d0000), // 81
    f64::from_bits(0x3fef6310b081992e), f64::from_bits(0x3f93cea3c4c28000), // 82
    f64::from_bits(0x3fef25f63ceeadcd), f64::from_bits(0x3f9b9fc114890000), // 83
    f64::from_bits(0x3feee9c8039113e7), f64::from_bits(0x3fa1b0d8ce110000), // 84
    f64::from_bits(0x3feeae8078cbb1ab), f64::from_bits(0x3fa58a5bd001c000), // 85
    f64::from_bits(0x3fee741aa29d0c9b), f64::from_bits(0x3fa95c8340d88000), // 86
    f64::from_bits(0x3fee3a91830a99b5), f64::from_bits(0x3fad276aef578000), // 87
    f64::from_bits(0x3fee01e009609a56), f64::from_bits(0x3fb07598e598c000), // 88
    f64::from_bits(0x3fedca01e577bb98), f64::from_bits(0x3fb253f5e30d2000), // 89
    f64::from_bits(0x3fed92f20b7c9103), f64::from_bits(0x3fb42edd8b380000), // 90
    f64::from_bits(0x3fed5cac66fb5cce), f64::from_bits(0x3fb606598757c000), // 91
    f64::from_bits(0x3fed272caa5ede9d), f64::from_bits(0x3fb7da76356a0000), // 92
    f64::from_bits(0x3fecf26e3e6b2ccd), f64::from_bits(0x3fb9ab434e1c6000), // 93
    f64::from_bits(0x3fecbe6da2a77902), f64::from_bits(0x3fbb78c7bb0d6000), // 94
    f64::from_bits(0x3fec8b266d37086d), f64::from_bits(0x3fbd431332e72000), // 95
    f64::from_bits(0x3fec5894bd5d5804), f64::from_bits(0x3fbf0a3171de6000), // 96
    f64::from_bits(0x3fec26b533bb9f8c), f64::from_bits(0x3fc067152b914000), // 97
    f64::from_bits(0x3febf583eeece73f), f64::from_bits(0x3fc147858292b000), // 98
    f64::from_bits(0x3febc4fd75db96c1), f64::from_bits(0x3fc2266ecdca3000), // 99
    f64::from_bits(0x3feb951e0c864a28), f64::from_bits(0x3fc303d7a6c55000), // 100
    f64::from_bits(0x3feb65e2c5ef3e2c), f64::from_bits(0x3fc3dfc33c331000), // 101
    f64::from_bits(0x3feb374867c9888b), f64::from_bits(0x3fc4ba366b7a8000), // 102
    f64::from_bits(0x3feb094b211d304a), f64::from_bits(0x3fc5933928d1f000), // 103
    f64::from_bits(0x3feadbe885f2ef7e), f64::from_bits(0x3fc66acd2418f000), // 104
    f64::from_bits(0x3feaaf1d31603da2), f64::from_bits(0x3fc740f8ec669000), // 105
    f64::from_bits(0x3fea82e63fd358a7), f64::from_bits(0x3fc815c0f51af000), // 106
    f64::from_bits(0x3fea5740ef09738b), f64::from_bits(0x3fc8e92954f68000), // 107
    f64::from_bits(0x3fea2c2a90ab4b27), f64::from_bits(0x3fc9bb3602f84000), // 108
    f64::from_bits(0x3fea01a01393f2d1), f64::from_bits(0x3fca8bed1c2c0000), // 109
    f64::from_bits(0x3fe9d79f24db3c1b), f64::from_bits(0x3fcb5b515c01d000), // 110
    f64::from_bits(0x3fe9ae2505c7b190), f64::from_bits(0x3fcc2967ccbcc000), // 111
    f64::from_bits(0x3fe9852ef297ce2f), f64::from_bits(0x3fccf635d5486000), // 112
    f64::from_bits(0x3fe95cbaeea44b75), f64::from_bits(0x3fcdc1bd3446c000), // 113
    f64::from_bits(0x3fe934c69de74838), f64::from_bits(0x3fce8c01b8cfe000), // 114
    f64::from_bits(0x3fe90d4f2f6752e6), f64::from_bits(0x3fcf5509c0179000), // 115
    f64::from_bits(0x3fe8e6528effd79d), f64::from_bits(0x3fd00e6c121fb800), // 116
    f64::from_bits(0x3fe8bfce9fcc007c), f64::from_bits(0x3fd071b80e93d000), // 117
    f64::from_bits(0x3fe899c0dabec30e), f64::from_bits(0x3fd0d46b9e867000), // 118
    f64::from_bits(0x3fe87427aa2317fb), f64::from_bits(0x3fd13687334bd000), // 119
    f64::from_bits(0x3fe84f00acb39a08), f64::from_bits(0x3fd1980d67234800), // 120
    f64::from_bits(0x3fe82a49e8653e55), f64::from_bits(0x3fd1f8ffe0cc8000), // 121
    f64::from_bits(0x3fe8060195f40260), f64::from_bits(0x3fd2595fd7636800), // 122
    f64::from_bits(0x3fe7e22563e0a329), f64::from_bits(0x3fd2b9300914a800), // 123
    f64::from_bits(0x3fe7beb377dcb5ad), f64::from_bits(0x3fd3187210436000), // 124
    f64::from_bits(0x3fe79baa679725c2), f64::from_bits(0x3fd377266dec1800), // 125
    f64::from_bits(0x3fe77907f2170657), f64::from_bits(0x3fd3d54ffbaf3000), // 126
    f64::from_bits(0x3fe756cadbd6130c), f64::from_bits(0x3fd432eee32fe000), // 127
];
