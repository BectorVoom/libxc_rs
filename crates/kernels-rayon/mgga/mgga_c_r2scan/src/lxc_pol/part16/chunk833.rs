//! MGGA_C_R2SCAN lxc pol — lxc_pol part 16 (v4rho3sigma_6) CSE chunk 833/1264 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part16_v4rho3sigma_6_chunk833(t495: f64, t8778: f64, t360: f64, t277: f64, t2892: f64, t571: f64, t7983: f64, t2573: f64, t2551: f64, t2562: f64, t2654: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t8779 = t8778 * t495;
    let t8780 = t360 * t8779;
    let t8783 = t277 * t2892;
    let t8784 = t8783 * t495;
    let t8785 = t360 * t8784;
    let t8792 = t571 * t7983;
    let t8795 = t8778 * t2573;
    let t8796 = t360 * t8795;
    let t8799 = t8778 * t2551;
    let t8800 = t360 * t8799;
    let t8803 = t2562 * t2654;
    let t8804 = t360 * t8803;
    (t8779, t8780, t8783, t8784, t8785, t8792, t8795, t8796, t8799, t8800, t8803, t8804)
}
