//! GGA_C_GAPC lxc pol — lxc_pol part 28 (v4rho2sigma2_7) CSE chunk 1365/1429 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part28_v4rho2sigma2_7_chunk1365<F: Float>(t209: F, t36345: F, t36359: F, t36374: F, t36389: F, t36420: F, t36434: F, t36449: F, t3537: F, t8598: F, t12291: F, t7056: F) -> (F, F, F) {
    let t36453 = (t36345 + t36359 + t36374 + t36389 + t36420 + t36434 + t36449) * t209;
    let t36455 = F::new(2.0) * t8598 * t3537;
    let t36457 = F::new(4.0) * t7056 * t12291;
    (t36453, t36455, t36457)
}
