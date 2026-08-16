//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 24 (v4rho3sigma_0) CSE chunk 869/1438 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part24_v4rho3sigma_0_chunk869(t2427: f64, t2655: f64, t152: f64, t31: f64, t185: f64, t9288: f64, t2448: f64, t67: f64, t758: f64, t2368: f64, t2505: f64, t745: f64) -> (f64, f64, f64, f64) {
    let t9896 = 12.0_f64 * t2427 * t2655;
    let t9897 = t31 * t152;
    let t9898 = t185 * t9288;
    let t9900 = 24.0_f64 * t9897 * t9898;
    let t9901 = t2448 * t67;
    let t9902 = t9901 * t758;
    let t9903 = 0.54934341918019635162e-3_f64 * t9902;
    let t9905 = t2368 * t745 * t2505;
    (t9896, t9900, t9903, t9905)
}
