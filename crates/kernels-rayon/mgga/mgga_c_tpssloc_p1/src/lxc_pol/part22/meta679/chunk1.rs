//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2242/2721 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2242(t1020: f64, t10508: f64, t248: f64, t5867: f64, t3039: f64, t5878: f64, t1041: f64, t13969: f64, t17696: f64, t10422: f64, t17648: f64, t3070: f64) -> (f64, f64, f64, f64) {
    let t62177 = t1020 * t248 * t10508 * t5867;
    let t62183 = t3039 * t248 * t10508 * t5878;
    let t62210 = t1041 * t13969 * t17696;
    let t62234 = t3070 * t10422 * t17648;
    (t62177, t62183, t62210, t62234)
}
