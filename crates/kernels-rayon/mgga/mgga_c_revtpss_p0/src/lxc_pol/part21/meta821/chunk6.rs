//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 3044/3259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3044(t1100: f64, t1102: f64, t12190: f64, t15562: f64, t16612: f64, t198: f64, t3329: f64, t3336: f64, t336: f64, t5023: f64, t5024: f64, t52762: f64, t52806: f64, t52808: f64, t53011: f64, t53056: f64, t53107: f64, t53163: f64, t53217: f64, t54238: f64, t54240: f64, t54242: f64, t54245: f64, t55405: f64, t55453: f64, t56099: f64) -> f64 {
    let t56115 = t198 * t336 * (t53011 + t53056 + t53107 + t53163 + t53217 + t55405 + t55453 + t56099) * t1102 + t52762 - t52806 - 3.0_f64 * t5023 * t16612 * t3336 * t1100 + t52808 - t54238 - 3.0_f64 * t5023 * t15562 * t3329 - t5023 * t5024 * t12190 - t54240 - t54242 - t54245;
    t56115
}
