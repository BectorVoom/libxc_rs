//! MGGA_C_REVTPSS lxc pol kernel — _part30_v4rho3sigma_5 meta624 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;
mod chunk6;
mod chunk7;
mod chunk8;
mod chunk9;
mod chunk10;
mod chunk11;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2150;
use chunk1::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2151;
use chunk2::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2152;
use chunk3::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2153;
use chunk4::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2154;
use chunk5::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2155;
use chunk6::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2156;
use chunk7::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2157;
use chunk8::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2158;
use chunk9::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2159;
use chunk10::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2160;
use chunk11::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2161;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_meta624(t99021: f64, t4452: f64, t92951: f64, t14719: f64, t25227: f64, t2661: f64, t14723: f64, t14774: f64, t7045: f64, t25266: f64, t4426: f64, t1561: f64, t93048: f64, t99009: f64, t99012: f64, t99013: f64, t99015: f64, t99017: f64, t99020: f64, t14741: f64, t1945: f64, t807: f64, t10886: f64, t4416: f64, t7028: f64, t27221: f64, t50789: f64, t50931: f64, t1549: f64, t92968: f64, t14697: f64, t25270: f64, t14693: f64, t14927: f64, t27261: f64, t93001: f64, t92996: f64, t92998: f64, t93000: f64, t10778: f64, t1941: f64, t50538: f64, t93016: f64, t25222: f64, t4435: f64, t14868: f64, t93082: f64, t14751: f64, t14757: f64, t25234: f64, t14738: f64, t7038: f64, t14732: f64, t25245: f64, t93004: f64, t93008: f64, t93010: f64, t93013: f64, t93021: f64, t14668: f64, t14933: f64, t2482: f64, t25260: f64, t814: f64, t2689: f64, t27239: f64, t93026: f64, t93028: f64, t93031: f64, t93035: f64, t93043: f64, t93045: f64, t93049: f64, t93055: f64, t93058: f64, t25277: f64, t4458: f64, t14685: f64, t14756: f64, t7021: f64, t14760: f64, t93015: f64, t93067: f64, t93069: f64, t93073: f64, t93077: f64, t93080: f64, t93084: f64, t93086: f64, t93088: f64, t93091: f64, t93095: f64, t98959: f64, t98981: f64, t99008: f64, t27316: f64, t686: f64, t72: f64, t25375: f64, t25387: f64, t2723: f64, t836: f64, t886: f64, t14978: f64, t15038: f64, t1558: f64, t1949: f64, t1956: f64, t1957: f64, t231: f64, t233: f64, t25317: f64, t25349: f64, t25391: f64, t25419: f64, t27199: f64, t27275: f64, t27357: f64, t2828: f64, t7053: f64, t7070: f64, t7071: f64, t7076: f64, t7083: f64, t7769: f64, t93112: f64, t93116: f64, t93124: f64, t98922: f64, t27182: f64, t2435: f64, t27334: f64, t10867: f64, t14485: f64, t25399: f64, t27195: f64, t1955: f64, t27198: f64, t2769: f64, t213: f64, t225: f64, t25392: f64, t25395: f64, t257: f64, t27353: f64, t4533: f64, t51525: f64, t51570: f64, t7048: f64, t7770: f64, t93126: f64, t93138: f64, t93142: f64, t93143: f64, t93147: f64, t93151: f64) -> (f64, f64, f64) {
        let (t99022, t99024, t99027, t99030, t99031, t99034, t99035) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2150(t99021, t4452, t92951, t14719, t25227, t2661, t14723, t14774, t7045, t25266, t4426, t1561, t93048);
        let t99037 = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2151(t99009, t99012, t99013, t99015, t99017, t99020, t99022, t99024, t99027, t99030, t99031, t99034, t99035);
        let (t99042, t99044, t99046, t99048, t99050, t99052) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2152(t14741, t1945, t807, t10886, t4416, t7028, t27221, t50789, t50931, t1549, t92968, t14697, t25270);
        let t99059 = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2153(t14693, t25270, t14927, t27261, t93001, t92996, t92998, t93000, t99042, t99044, t99046, t99048, t99050, t99052);
        let (t99063, t99065, t99066, t99070, t99071, t99073) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2154(t10778, t1941, t50538, t93016, t25222, t4435, t14868, t2661, t93082, t14751, t7045, t14757, t25234);
        let t99079 = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2155(t99073, t14738, t7038, t14732, t25245, t93004, t93008, t93010, t93013, t93021, t99063, t99065, t99066, t99070, t99071);
        let t99098 = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2156(t14668, t27261, t14933, t2482, t25260, t814, t2689, t27239, t93026, t93028, t93031, t93035, t93043, t93045, t93049, t93055, t93058);
        let t99116 = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2157(t25277, t4458, t14685, t14756, t7021, t14760, t93015, t93067, t93069, t93073, t93077, t93080, t93084, t93086, t93088, t93091, t93095);
        let (t99119, t99125, t99127) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2158(t98959, t98981, t99008, t99037, t99059, t99079, t99098, t99116, t27316, t686, t72, t25375);
        let t99159 = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2159(t25387, t99125, t2723, t836, t886, t14978, t15038, t1558, t1949, t1956, t1957, t231, t233, t25317, t25349, t25391, t25419, t27199, t27275, t27357, t2828, t7053, t7070, t7071, t7076, t7083, t7769, t93112, t93116, t93124, t98922, t99119, t99127);
        let (t99161, t99163, t99166, t99174, t99186, t99188, t99191) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2160(t27182, t686, t72, t25387, t2435, t27334, t10867, t1949, t14485, t25399, t27195, t1955, t27198, t2769);
        let t99194 = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2161(t213, t225, t25392, t25395, t257, t27353, t4533, t51525, t51570, t7048, t7070, t7071, t7770, t93126, t93138, t93142, t93143, t93147, t93151, t99119, t99163, t99166, t99174, t99186, t99188, t99191);
    (t99159, t99161, t99194)
}
