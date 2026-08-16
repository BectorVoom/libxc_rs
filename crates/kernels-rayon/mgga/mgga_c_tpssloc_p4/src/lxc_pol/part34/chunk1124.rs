//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 34 (v4rho3sigma_10) CSE chunk 1124/1250 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part34_v4rho3sigma_10_chunk1124(t22642: f64, t22643: f64, t7700: f64, t22716: f64, t7701: f64, t1834: f64, t212: f64, t6890: f64, t7733: f64, t81186: f64, t26392: f64, t80670: f64) -> (f64, f64, f64, f64, f64) {
    let t90642 = t22642 * t22643 * t7700;
    let t90659 = t22716 * t7701;
    let t90663 = t22642 * t212 * t1834 * t6890;
    let t90807 = t81186 * t7733;
    let t90837 = t80670 * t26392;
    (t90642, t90659, t90663, t90807, t90837)
}
