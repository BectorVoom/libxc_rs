//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2878/3259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2878(t41880: f64, t4595: f64, t15513: f64, t914: f64, t936: f64, t15416: f64, t2919: f64, t2923: f64, t4587: f64, t2927: f64, t11380: f64, t4590: f64) -> (f64, f64, f64, f64, f64) {
    let t52213 = 6.0_f64 * t41880 * t4595;
    let t52214 = t15513 * t914;
    let t52216 = 3.0_f64 * t52214 * t936;
    let t52218 = 3.0_f64 * t15416 * t2919;
    let t52219 = t4587 * t2923;
    let t52221 = 0.48245938496077605201e2_f64 * t52219 * t2927;
    let t52223 = 1.0_f64 * t4590 * t11380;
    (t52213, t52216, t52218, t52221, t52223)
}
