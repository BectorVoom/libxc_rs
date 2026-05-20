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

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

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
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_meta624<F: Float>(t99021: F, t4452: F, t92951: F, t14719: F, t25227: F, t2661: F, t14723: F, t14774: F, t7045: F, t25266: F, t4426: F, t1561: F, t93048: F, t99009: F, t99012: F, t99013: F, t99015: F, t99017: F, t99020: F, t14741: F, t1945: F, t807: F, t10886: F, t4416: F, t7028: F, t27221: F, t50789: F, t50931: F, t1549: F, t92968: F, t14697: F, t25270: F, t14693: F, t14927: F, t27261: F, t93001: F, t92996: F, t92998: F, t93000: F, t10778: F, t1941: F, t50538: F, t93016: F, t25222: F, t4435: F, t14868: F, t93082: F, t14751: F, t14757: F, t25234: F, t14738: F, t7038: F, t14732: F, t25245: F, t93004: F, t93008: F, t93010: F, t93013: F, t93021: F, t14668: F, t14933: F, t2482: F, t25260: F, t814: F, t2689: F, t27239: F, t93026: F, t93028: F, t93031: F, t93035: F, t93043: F, t93045: F, t93049: F, t93055: F, t93058: F, t25277: F, t4458: F, t14685: F, t14756: F, t7021: F, t14760: F, t93015: F, t93067: F, t93069: F, t93073: F, t93077: F, t93080: F, t93084: F, t93086: F, t93088: F, t93091: F, t93095: F, t98959: F, t98981: F, t99008: F, t27316: F, t686: F, t72: F, t25375: F, t25387: F, t2723: F, t836: F, t886: F, t14978: F, t15038: F, t1558: F, t1949: F, t1956: F, t1957: F, t231: F, t233: F, t25317: F, t25349: F, t25391: F, t25419: F, t27199: F, t27275: F, t27357: F, t2828: F, t7053: F, t7070: F, t7071: F, t7076: F, t7083: F, t7769: F, t93112: F, t93116: F, t93124: F, t98922: F, t27182: F, t2435: F, t27334: F, t10867: F, t14485: F, t25399: F, t27195: F, t1955: F, t27198: F, t2769: F, t213: F, t225: F, t25392: F, t25395: F, t257: F, t27353: F, t4533: F, t51525: F, t51570: F, t7048: F, t7770: F, t93126: F, t93138: F, t93142: F, t93143: F, t93147: F, t93151: F) -> (F, F, F) {
        let (t99022, t99024, t99027, t99030, t99031, t99034, t99035) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2150::<F>(t99021, t4452, t92951, t14719, t25227, t2661, t14723, t14774, t7045, t25266, t4426, t1561, t93048);
        let t99037 = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2151::<F>(t99009, t99012, t99013, t99015, t99017, t99020, t99022, t99024, t99027, t99030, t99031, t99034, t99035);
        let (t99042, t99044, t99046, t99048, t99050, t99052) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2152::<F>(t14741, t1945, t807, t10886, t4416, t7028, t27221, t50789, t50931, t1549, t92968, t14697, t25270);
        let t99059 = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2153::<F>(t14693, t25270, t14927, t27261, t93001, t92996, t92998, t93000, t99042, t99044, t99046, t99048, t99050, t99052);
        let (t99063, t99065, t99066, t99070, t99071, t99073) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2154::<F>(t10778, t1941, t50538, t93016, t25222, t4435, t14868, t2661, t93082, t14751, t7045, t14757, t25234);
        let t99079 = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2155::<F>(t99073, t14738, t7038, t14732, t25245, t93004, t93008, t93010, t93013, t93021, t99063, t99065, t99066, t99070, t99071);
        let t99098 = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2156::<F>(t14668, t27261, t14933, t2482, t25260, t814, t2689, t27239, t93026, t93028, t93031, t93035, t93043, t93045, t93049, t93055, t93058);
        let t99116 = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2157::<F>(t25277, t4458, t14685, t14756, t7021, t14760, t93015, t93067, t93069, t93073, t93077, t93080, t93084, t93086, t93088, t93091, t93095);
        let (t99119, t99125, t99127) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2158::<F>(t98959, t98981, t99008, t99037, t99059, t99079, t99098, t99116, t27316, t686, t72, t25375);
        let t99159 = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2159::<F>(t25387, t99125, t2723, t836, t886, t14978, t15038, t1558, t1949, t1956, t1957, t231, t233, t25317, t25349, t25391, t25419, t27199, t27275, t27357, t2828, t7053, t7070, t7071, t7076, t7083, t7769, t93112, t93116, t93124, t98922, t99119, t99127);
        let (t99161, t99163, t99166, t99174, t99186, t99188, t99191) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2160::<F>(t27182, t686, t72, t25387, t2435, t27334, t10867, t1949, t14485, t25399, t27195, t1955, t27198, t2769);
        let t99194 = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2161::<F>(t213, t225, t25392, t25395, t257, t27353, t4533, t51525, t51570, t7048, t7070, t7071, t7770, t93126, t93138, t93142, t93143, t93147, t93151, t99119, t99163, t99166, t99174, t99186, t99188, t99191);
    (t99159, t99161, t99194)
}
