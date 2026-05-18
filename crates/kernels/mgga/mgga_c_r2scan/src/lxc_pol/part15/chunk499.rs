//! MGGA_C_R2SCAN lxc pol — lxc_pol part 15 (v4rho3sigma_5) CSE chunk 499/1253 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part15_v4rho3sigma_5_chunk499<F: Float>(t1551: F, t1554: F, t1556: F, t1562: F, t1563: F, t2259: F, t285: F, t495: F, t499: F) -> F {
    let t2262 = t1551 * t285 + t1554 * t285 + t495 * t1556 / F::new(2.0) - F::new(5.0) / F::new(16.0) * t1562 * t1563 + t499 * t2259 / F::new(4.0);
    t2262
}
