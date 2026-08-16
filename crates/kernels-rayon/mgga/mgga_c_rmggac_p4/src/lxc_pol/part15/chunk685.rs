//! MGGA_C_RMGGAC lxc pol — lxc_pol part 15 (v4rho3sigma_6) CSE chunk 685/1110 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part15_v4rho3sigma_6_chunk685(t1356: f64, t9867: f64, t2301: f64, t2868: f64, t1734: f64, t36: f64) -> (f64, f64, f64) {
    let t9868 = t1356 * t9867;
    let t9869 = 0.79828278012425390428e-1_f64 * t9868;
    let t9870 = t2868 * t2301;
    let t9871 = 0.2993560425465952141e-1_f64 * t9870;
    let t9872 = t36 * t1734;
    (t9869, t9871, t9872)
}
