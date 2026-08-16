//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta550 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2104;
use chunk1::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2105;
use chunk2::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2106;
use chunk3::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2107;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta550(t1388: f64, t14013: f64, t14024: f64, t1410: f64, t22179: f64, t22183: f64, t22255: f64, t22260: f64, t22264: f64, t22268: f64, t22271: f64, t22276: f64, t22281: f64, t5671: f64, t9953: f64, t6864: f64, t9918: f64, t1353: f64, t6816: f64, t4012: f64, t828: f64, t3930: f64, t6876: f64, t1883: f64, t5627: f64, t13783: f64, t13926: f64, t6869: f64, t13789: f64, t14038: f64, t14040: f64, t14042: f64, t14043: f64, t14049: f64, t14053: f64, t14057: f64, t3934: f64, t9977: f64, t22035: f64, t22065: f64, t22105: f64, t22140: f64, t22153: f64, t22176: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let t22284 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2104(t1388, t14013, t14024, t1410, t22179, t22183, t22255, t22260, t22264, t22268, t22271, t22276, t22281, t5671, t9953);
        let (t22285, t22287, t22289, t22292, t22294, t22295, t22298) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2105(t6864, t9918, t1353, t6816, t4012, t828, t3930, t6876, t1883, t5627, t13783, t13926, t6869);
        let (t22299, t22304) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2106(t13789, t22298, t14038, t14040, t14042, t14043, t14049, t14053, t14057, t1410, t22285, t22289, t22292, t22295, t3934, t9977);
        let t22307 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2107(t22035, t22065, t22105, t22140, t22153, t22176, t22284, t22304);
    (t22285, t22287, t22289, t22292, t22294, t22295, t22298, t22299, t22307)
}
