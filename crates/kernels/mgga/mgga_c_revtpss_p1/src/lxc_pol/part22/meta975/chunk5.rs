//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3281/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3281<F: Float>(t39799: F, t39807: F, t39813: F, t39818: F, t39823: F, t40084: F, t40088: F, t40099: F, t61170: F, t61171: F, t61172: F, t61173: F, t61177: F, t61179: F, t61181: F, t61190: F, t61191: F, t61197: F, t61198: F, t61199: F) -> F {
    let t62263 = t61170 + t61171 + t61172 - t61173 + t39799 + t61177 + t61179 + t39807 - t39813 + t61181 - t39818 - t39823 + t61190 - t61191 + t40084 + t61197 + t40088 - t61198 + t61199 + t40099;
    t62263
}
