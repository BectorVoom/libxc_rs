//! GGA_C_GAPC lxc pol — lxc_pol part 30 (v4rho2sigma2_9) CSE chunk 1330/1331 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part30_v4rho2sigma2_9_chunk1330<F: Float>(t12435: F, t12664: F, t37331: F, t37332: F, t37333: F, t37334: F, t37335: F, t37336: F, t37337: F, t38525: F, t38526: F, t38527: F, t38528: F, t38530: F, t38859: F, t38860: F, t7: F) -> F {
    let tv4rho2sigma29 = t37331 - t37332 + t37333 - t37334 + t37335 - t37336 - t37337 + t38525 - t38526 - t38527 + t38528 + t12435 + t38530 + t12664 + t7 * (t38859 + t38860);
    tv4rho2sigma29
}
