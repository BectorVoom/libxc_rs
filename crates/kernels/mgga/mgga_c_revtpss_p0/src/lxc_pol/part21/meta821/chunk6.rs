//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 3044/3259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3044<F: Float>(t1100: F, t1102: F, t12190: F, t15562: F, t16612: F, t198: F, t3329: F, t3336: F, t336: F, t5023: F, t5024: F, t52762: F, t52806: F, t52808: F, t53011: F, t53056: F, t53107: F, t53163: F, t53217: F, t54238: F, t54240: F, t54242: F, t54245: F, t55405: F, t55453: F, t56099: F) -> F {
    let t56115 = t198 * t336 * (t53011 + t53056 + t53107 + t53163 + t53217 + t55405 + t55453 + t56099) * t1102 + t52762 - t52806 - F::cast_from(3.0_f64) * t5023 * t16612 * t3336 * t1100 + t52808 - t54238 - F::cast_from(3.0_f64) * t5023 * t15562 * t3329 - t5023 * t5024 * t12190 - t54240 - t54242 - t54245;
    t56115
}
