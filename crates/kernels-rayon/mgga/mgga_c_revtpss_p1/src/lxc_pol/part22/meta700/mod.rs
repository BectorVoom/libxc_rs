//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta700 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2710;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2711;
use chunk2::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2712;
use chunk3::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2713;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta700(t6864: f64, t9918: f64, t1353: f64, t6816: f64, t4012: f64, t828: f64, t3930: f64, t6876: f64, t1883: f64, t5627: f64, t13783: f64, t13926: f64, t6869: f64, t13789: f64, t14038: f64, t14040: f64, t14042: f64, t14043: f64, t14049: f64, t14053: f64, t14057: f64, t1410: f64, t3934: f64, t9977: f64, t22035: f64, t22065: f64, t22105: f64, t22140: f64, t22153: f64, t22176: f64, t22284: f64, t6862: f64, t72: f64, t686: f64, t10023: f64, t1385: f64, t6888: f64, t10070: f64, t10074: f64, t1399: f64, t14191: f64, t14193: f64, t14203: f64, t14209: f64, t14255: f64, t213: f64, t21981: f64, t22005: f64, t22009: f64, t22016: f64, t4118: f64, t546: f64, t5659: f64, t5675: f64, t5745: f64, t5755: f64, t5767: f64, t6874: f64, t820: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t22285, t22287, t22289, t22292, t22294, t22295, t22298) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2710(t6864, t9918, t1353, t6816, t4012, t828, t3930, t6876, t1883, t5627, t13783, t13926, t6869);
        let (t22299, t22304) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2711(t13789, t22298, t14038, t14040, t14042, t14043, t14049, t14053, t14057, t1410, t22285, t22289, t22292, t22295, t3934, t9977);
        let t22307 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2712(t22035, t22065, t22105, t22140, t22153, t22176, t22284, t22304);
        let (t22314, t22315, t22321, t22325) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2713(t6862, t72, t686, t10023, t1385, t6888, t10070, t10074, t1399, t14191, t14193, t14203, t14209, t14255, t1883, t213, t21981, t22005, t22009, t22016, t22307, t4118, t546, t5659, t5675, t5745, t5755, t5767, t6874, t820);
    (t22287, t22289, t22294, t22295, t22298, t22299, t22307, t22314, t22315, t22321, t22325)
}
