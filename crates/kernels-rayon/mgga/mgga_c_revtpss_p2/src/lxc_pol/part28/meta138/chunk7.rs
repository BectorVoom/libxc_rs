//! MGGA_C_REVTPSS lxc pol — lxc_pol part 28 (v4rho3sigma_3) CSE chunk 762/2277 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk762(t2846: f64, t2848: f64, t2855: f64, t2860: f64, t2864: f64) -> (f64, f64) {
    let t3070 = 0.19755555555555555556e-1_f64 * t2846;
    let t3075 = t3070 + 0.9877777777777777778e-2_f64 * t2848 - 0.9877777777777777778e-2_f64 * t2855 + 0.29633333333333333334e-1_f64 * t2860 - 0.14816666666666666667e-1_f64 * t2864;
    (t3070, t3075)
}
