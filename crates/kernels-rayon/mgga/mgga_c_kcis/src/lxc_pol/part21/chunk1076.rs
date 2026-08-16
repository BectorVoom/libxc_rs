//! MGGA_C_KCIS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 1076/1389 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part21_v4rho3sigma_3_chunk1076(t3045: f64, t7691: f64, t5329: f64, t3040: f64, t7709: f64, t2836: f64, t3489: f64) -> (f64, f64, f64, f64, f64) {
    let t26731 = t7691 * t3045;
    let t26732 = t5329 * t26731;
    let t26735 = t7709 * t3040;
    let t26736 = t5329 * t26735;
    let t26739 = t2836 * t3489;
    (t26731, t26732, t26735, t26736, t26739)
}
