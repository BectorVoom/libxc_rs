//! MGGA_C_RMGGAC lxc pol — lxc_pol part 14 (v4rho3sigma_5) CSE chunk 582/1089 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part14_v4rho3sigma_5_chunk582(t7617: f64, t854: f64, t305: f64, t830: f64, t2100: f64, t7587: f64, t2103: f64, t7591: f64, t22: f64, t3851: f64, t36: f64, t794: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t7625 = t854 * t7617;
    let t7627 = t305 * t830;
    let t7628 = 0.48783947674259960818e-1_f64 * t7627;
    let t7629 = t2100 * t7587;
    let t7631 = t2103 * t7591;
    let t7633 = t3851 * t22;
    let t7634 = t36 * t794;
    (t7625, t7628, t7629, t7631, t7633, t7634)
}
