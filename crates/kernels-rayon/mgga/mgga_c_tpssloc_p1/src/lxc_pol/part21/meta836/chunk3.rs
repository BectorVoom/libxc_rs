//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2973/3221 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2973(t10422: f64, t17704: f64, t3070: f64, t17680: f64, t1041: f64, t13969: f64, t17692: f64, t10408: f64, t10413: f64, t10937: f64, t17697: f64, t17705: f64, t17984: f64, t2771: f64, t3048: f64, t42334: f64, t42388: f64, t42586: f64, t4575: f64, t4600: f64, t48477: f64, t48607: f64, t48611: f64, t48612: f64, t49690: f64, t49692: f64, t49697: f64, t49771: f64, t49984: f64, t5878: f64, t61098: f64) -> f64 {
    let t62013 = t3070 * t10422 * t17704;
    let t62032 = t3070 * t10422 * t17680;
    let t62038 = t1041 * t13969 * t17692;
    let t62042 = -t49771 * t4600 / 768.0_f64 - t10937 * t17705 / 216.0_f64 + t62013 / 1728.0_f64 - t42586 / 6912.0_f64 - t49690 / 3456.0_f64 - t49692 / 5184.0_f64 - t42334 * t17984 / 256.0_f64 - 5.0_f64 / 576.0_f64 * t48607 * t10408 * t61098 - 5.0_f64 / 13824.0_f64 * t10413 * t10408 * t5878 * t2771 + t42388 * t48611 * t48612 * t48477 / 128.0_f64 + t62032 / 3456.0_f64 - t49984 * t4575 / 216.0_f64 + t49697 / 1728.0_f64 + 5.0_f64 / 5184.0_f64 * t62038 - 5.0_f64 / 486.0_f64 * t3048 * t17697;
    t62042
}
