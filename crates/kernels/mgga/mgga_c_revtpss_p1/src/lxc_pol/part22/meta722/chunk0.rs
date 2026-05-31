//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 2773/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2773<F: Float>(t2552: F, t2555: F, t2564: F, t2577: F, t689: F, t700: F) -> (F, F, F) {
    let t40056 = t2552 * t2552;
    let t40059 = t2555 * t2555;
    let t40067 = F::cast_from(0.4274e0_f64) * t689 * t2564 * t700 * t2577;
    (t40056, t40059, t40067)
}
