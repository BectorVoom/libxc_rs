//! GGA_C_GAPLOC lxc pol — lxc_pol part 33 (v4rho2sigma2_16) CSE chunk 1262/1294 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part33_v4rho2sigma2_16_chunk1262<F: Float>(t12251: F, t1980: F, t12161: F, t296: F, t1: F, t787: F, t12244: F, t2028: F, t28593: F, t33368: F, t33376: F, t33381: F, t33385: F, t33387: F, t33389: F, t33392: F, t33394: F, t33397: F, t33405: F, t33409: F, t5669: F) -> (F, F) {
    let t39118 = t1980 * t12251;
    let t39121 = t296 * t12161;
    let t39123 = t787 * t39121 * t1;
    let t39126 = -t33368 + 0.1022478025437886658e1 * t5669 * t12244 + t28593 - 0.79445533226334281486e-1 * t39118 * t2028 - 0.79445533226334281486e-1 * t39123 * t2028 + t33376 + t33381 - t33385 + t33387 - t33389 + t33392 + t33394 - t33397 - t33405 - t33409;
    (t39121, t39126)
}
