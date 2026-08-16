//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 1871/2721 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1871(t20856: f64, t9975: f64, t10080: f64, t2632: f64, t2728: f64, t13416: f64, t5585: f64, t232: f64, t860: f64, t1510: f64, t17030: f64, t4295: f64, t5617: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t20857 = t20856 * t9975;
    let t20858 = t10080 * t20857;
    let t20861 = t20856 * t2632;
    let t20862 = t2728 * t20861;
    let t20867 = t13416 * t5585;
    let t20870 = t20856 * t232;
    let t20871 = t860 * t20870;
    let t20873 = t17030 * t1510;
    let t20876 = t4295 * t5617;
    (t20857, t20858, t20861, t20862, t20867, t20870, t20871, t20873, t20876)
}
