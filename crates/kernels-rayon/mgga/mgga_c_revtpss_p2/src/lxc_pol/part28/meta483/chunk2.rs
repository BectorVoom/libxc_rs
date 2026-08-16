//! MGGA_C_REVTPSS lxc pol — lxc_pol part 28 (v4rho3sigma_3) CSE chunk 1836/2277 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1836(t3151: f64, t3318: f64, t7168: f64, t1035: f64, t7135: f64, t1043: f64, t1089: f64, t3133: f64, t1976: f64, t3046: f64) -> (f64, f64, f64, f64, f64) {
    let t25678 = t7168 * t3151 * t3318;
    let t25681 = t1035 * t7135;
    let t25683 = t25681 * t1043 * t1089;
    let t25687 = t7168 * t3133 * t1089;
    let t25692 = t3046 * t1976;
    (t25678, t25681, t25683, t25687, t25692)
}
