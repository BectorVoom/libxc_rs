//! GGA_C_ACGGAP lxc pol — lxc_pol part 11 (v4rho3sigma_3) CSE chunk 729/1213 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part11_v4rho3sigma_3_chunk729(t2082: f64, t7676: f64, t1017: f64, t137: f64, t1089: f64, t5011: f64, t598: f64, t2116: f64, t950: f64, t151: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t7677 = t7676 * t2082;
    let t7678 = 0.12862205435420921092e-2_f64 * t7677;
    let t7679 = t137 * t1017;
    let t7681 = t1089 * t5011 * t7679;
    let t7682 = t598 * t7681;
    let t7684 = t2116 * t950;
    let t7685 = t151 * t7684;
    (t7678, t7679, t7681, t7682, t7684, t7685)
}
