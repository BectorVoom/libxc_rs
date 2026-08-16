//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3925/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3925<F: Float>(t5778: F, t198: F, t22466: F, t3829: F, t40076: F, t40079: F, t4147: F, t47152: F, t532: F, t5536: F, t74145: F, t74146: F, t74147: F, t74148: F, t74149: F, t74150: F, t74151: F) -> F {
    let t75416 = t5778 * t5778;
    let t75421 = -F::cast_from(2.0_f64) * t198 * t4147 * t532 * t75416 - F::cast_from(6.0_f64) * t22466 * t3829 * t5536 + t40076 - t40079 + t47152 - t74145 - t74146 - t74147 - t74148 - t74149 - t74150 + t74151;
    t75421
}
