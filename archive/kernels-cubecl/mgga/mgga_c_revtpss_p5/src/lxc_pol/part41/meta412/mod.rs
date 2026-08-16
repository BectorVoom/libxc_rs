//! MGGA_C_REVTPSS lxc pol kernel — _part41_v4rho3tau_4 meta412 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk1443;
use chunk1::mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk1444;
use chunk2::mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk1445;
use chunk3::mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk1446;
use chunk4::mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk1447;
use chunk5::mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk1448;
use chunk6::mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk1449;
use chunk7::mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk1450;
use chunk8::mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk1451;
use chunk9::mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk1452;
use chunk10::mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk1453;
use chunk11::mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk1454;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_meta412<F: Float>(t221: F, t4019: F, t6874: F, t4018: F, t22079: F, t5673: F, t5675: F, t1353: F, t6836: F, t828: F, t9942: F, t1868: F, t5591: F, t4012: F, t1388: F, t14013: F, t14024: F, t1410: F, t22179: F, t22183: F, t22255: F, t22260: F, t22264: F, t5671: F, t9953: F, t6864: F, t9918: F, t6816: F, t3930: F, t6876: F, t1883: F, t5627: F, t13783: F, t13926: F, t6869: F, t13789: F, t14038: F, t14040: F, t14042: F, t14043: F, t14049: F, t14053: F, t14057: F, t3934: F, t9977: F, t22035: F, t22065: F, t22105: F, t22140: F, t22153: F, t22176: F, t6862: F, t72: F, t686: F, t10023: F, t1385: F, t6888: F, t10070: F, t10074: F, t1399: F, t14191: F, t14193: F, t14203: F, t14209: F, t14255: F, t213: F, t21981: F, t22005: F, t22009: F, t22016: F, t4118: F, t546: F, t5659: F, t5745: F, t5755: F, t5767: F, t820: F, t14239: F, t5741: F, t6844: F, t4101: F, t10098: F, t10102: F, t10109: F, t10114: F, t14218: F, t14221: F, t14227: F, t14229: F, t14233: F, t14241: F, t14243: F, t545: F, t869: F, t689: F, t4003: F, t5744: F, t2782: F, t4086: F, t543: F, t1432: F, t10049: F, t10117: F, t10126: F, t10129: F, t10137: F, t10143: F, t14252: F, t1437: F, t22253: F, t5735: F, t21998: F, t1427: F, t13727: F, t13733: F, t13737: F, t1424: F, t1445: F, t4071: F, t5715: F, t5775: F, t6896: F, t9632: F, t9639: F, t9642: F, t9650: F, t9666: F, t1903: F, t5774: F, t4076: F, t6918: F, t3915: F, t6889: F, t786: F, t1364: F, t14100: F, t5722: F, t1357: F, t6919: F, t1444: F, t14081: F, t14084: F, t14087: F, t14299: F, t1904: F, t9677: F, t9687: F, t9691: F) -> (F, F, F) {
        let (t22268, t22271, t22276, t22279) = mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk1443::<F>(t221, t4019, t6874, t4018, t22079, t5673, t5675, t1353, t6836, t828, t9942, t1868, t5591);
        let t22284 = mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk1444::<F>(t22279, t4012, t828, t1388, t14013, t14024, t1410, t22179, t22183, t22255, t22260, t22264, t22268, t22271, t22276, t5671, t9953);
        let (t22285, t22289, t22292, t22295, t22298) = mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk1445::<F>(t6864, t9918, t1353, t6816, t4012, t828, t3930, t6876, t1883, t5627, t13783, t13926, t6869);
        let t22304 = mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk1446::<F>(t13789, t22298, t14038, t14040, t14042, t14043, t14049, t14053, t14057, t1410, t22285, t22289, t22292, t22295, t3934, t9977);
        let (t22307, t22316) = mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk1447::<F>(t22035, t22065, t22105, t22140, t22153, t22176, t22284, t22304, t6862, t72, t686, t10023);
        let t22325 = mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk1448::<F>(t1385, t6888, t10070, t10074, t1399, t14191, t14193, t14203, t14209, t14255, t1883, t213, t21981, t22005, t22009, t22016, t22307, t22316, t4118, t546, t5659, t5675, t5745, t5755, t5767, t6874, t820);
        let t22344 = mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk1449::<F>(t14239, t5741, t6844, t72, t686, t4101, t6874, t10098, t10102, t10109, t10114, t14218, t14221, t14227, t14229, t14233, t14241, t14243, t22005, t5675, t5745);
        let (t22353, t22362, t22366, t22369) = mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk1450::<F>(t545, t6888, t869, t689, t22005, t4003, t5744, t2782, t21981, t4086, t543, t22009);
        let t22384 = mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk1451::<F>(t22369, t2782, t22005, t4086, t543, t6888, t72, t1432, t686, t10049, t10117, t10126, t10129, t10137, t10143, t1399, t14252, t1437, t22009, t22253, t22353, t22362, t22366, t5659, t5735, t5755, t6862, t820);
        let t22393 = mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk1452::<F>(t21998, t22325, t22344, t22384, t1427, t213, t6888, t13727, t13733, t13737, t1424, t1445, t4071, t5715, t5775, t6896, t9632, t9639, t9642, t9650, t9666);
        let (t22395, t22400, t22405, t22407) = mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk1453::<F>(t1903, t5774, t4076, t6918, t72, t686, t3915, t6889, t786, t1364, t14100, t5722);
        let t22418 = mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk1454::<F>(t1357, t6919, t689, t1444, t6918, t4076, t14081, t14084, t14087, t1424, t14299, t1904, t22395, t22400, t22405, t22407, t9677, t9687, t9691);
    (t22307, t22393, t22418)
}
