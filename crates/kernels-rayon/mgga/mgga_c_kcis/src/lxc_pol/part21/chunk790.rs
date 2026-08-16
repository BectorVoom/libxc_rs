//! MGGA_C_KCIS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 790/1389 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part21_v4rho3sigma_3_chunk790(t9160: f64, t158: f64, t157: f64, t2491: f64, t812: f64, t2593: f64, t2585: f64, t2484: f64, t2618: f64, t2526: f64, t808: f64, t137: f64, t8998: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t9161 = 1.0_f64 / t9160;
    let t9162 = t158 * t9161;
    let t9163 = t157 * t9162;
    let t9165 = t812 * t2491;
    let t9166 = t2593 * t9165;
    let t9168 = t2585 * t812;
    let t9170 = t2484 * t2618;
    let t9172 = t812 * t2526;
    let t9173 = t808 * t9172;
    let t9175 = t8998 * t137;
    (t9161, t9163, t9166, t9168, t9170, t9173, t9175)
}
