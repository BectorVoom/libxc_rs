//! MGGA_C_REVTPSS lxc pol kernel — _part33_v4rho3sigma_8 meta475 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;
mod chunk6;
mod chunk7;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1727;
use chunk1::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1728;
use chunk2::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1729;
use chunk3::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1730;
use chunk4::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1731;
use chunk5::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1732;
use chunk6::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1733;
use chunk7::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1734;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_meta475(t6864: f64, t9918: f64, t1353: f64, t6816: f64, t4012: f64, t828: f64, t3930: f64, t6876: f64, t1883: f64, t5627: f64, t13783: f64, t13926: f64, t6869: f64, t13789: f64, t14038: f64, t14040: f64, t14042: f64, t14043: f64, t14049: f64, t14053: f64, t14057: f64, t1410: f64, t3934: f64, t9977: f64, t22035: f64, t22065: f64, t22105: f64, t22140: f64, t22153: f64, t22176: f64, t22284: f64, t6862: f64, t72: f64, t686: f64, t10023: f64, t1385: f64, t6888: f64, t10070: f64, t10074: f64, t1399: f64, t14191: f64, t14193: f64, t14203: f64, t14209: f64, t14255: f64, t213: f64, t21981: f64, t22005: f64, t22009: f64, t22016: f64, t4118: f64, t546: f64, t5659: f64, t5675: f64, t5745: f64, t5755: f64, t5767: f64, t6874: f64, t820: f64, t14239: f64, t5741: f64, t6844: f64, t4101: f64, t10098: f64, t10102: f64, t10109: f64, t10114: f64, t14218: f64, t14221: f64, t14227: f64, t14229: f64, t14233: f64, t14241: f64, t14243: f64, t545: f64, t869: f64, t689: f64, t4003: f64, t5744: f64, t2782: f64, t4086: f64, t543: f64, t1432: f64, t10049: f64, t10117: f64, t10126: f64, t10129: f64, t10137: f64, t10143: f64, t14252: f64, t1437: f64, t22253: f64, t5735: f64, t21998: f64, t1427: f64, t13727: f64, t13733: f64, t13737: f64, t1424: f64, t1445: f64, t4071: f64, t5715: f64, t5775: f64, t6896: f64, t9632: f64, t9639: f64, t9642: f64, t9650: f64, t9666: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t22285, t22287, t22289, t22292, t22295, t22298) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1727(t6864, t9918, t1353, t6816, t4012, t828, t3930, t6876, t1883, t5627, t13783, t13926, t6869);
        let (t22299, t22304) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1728(t13789, t22298, t14038, t14040, t14042, t14043, t14049, t14053, t14057, t1410, t22285, t22289, t22292, t22295, t3934, t9977);
        let (t22307, t22316) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1729(t22035, t22065, t22105, t22140, t22153, t22176, t22284, t22304, t6862, t72, t686, t10023);
        let t22325 = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1730(t1385, t6888, t10070, t10074, t1399, t14191, t14193, t14203, t14209, t14255, t1883, t213, t21981, t22005, t22009, t22016, t22307, t22316, t4118, t546, t5659, t5675, t5745, t5755, t5767, t6874, t820);
        let t22344 = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1731(t14239, t5741, t6844, t72, t686, t4101, t6874, t10098, t10102, t10109, t10114, t14218, t14221, t14227, t14229, t14233, t14241, t14243, t22005, t5675, t5745);
        let (t22353, t22362, t22366, t22369) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1732(t545, t6888, t869, t689, t22005, t4003, t5744, t2782, t21981, t4086, t543, t22009);
        let t22384 = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1733(t22369, t2782, t22005, t4086, t543, t6888, t72, t1432, t686, t10049, t10117, t10126, t10129, t10137, t10143, t1399, t14252, t1437, t22009, t22253, t22353, t22362, t22366, t5659, t5735, t5755, t6862, t820);
        let (t22386, t22387, t22393) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1734(t21998, t22325, t22344, t22384, t1427, t213, t6888, t13727, t13733, t13737, t1424, t1445, t4071, t5715, t5775, t6896, t9632, t9639, t9642, t9650, t9666);
    (t22287, t22289, t22295, t22299, t22307, t22386, t22387, t22393)
}
