//! GGA_C_GAPC lxc pol — lxc_pol part 24 (v4rho2sigma2_3) CSE chunk 1132/1133 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part24_v4rho2sigma2_3_chunk1132<F: Float>(t12337: F, t12335: F, t12330: F, t12347: F, t12575: F, t12435: F, t37331: F, t37332: F, t37333: F, t37334: F, t37335: F, t37336: F, t37337: F, t38519: F, t38522: F, t7: F) -> (F,) {
    let t38525 = 4.0 * t12337;
    let t38526 = 4.0 * t12335;
    let t38527 = 2.0 * t12330;
    let t38528 = 4.0 * t12347;
    let t38530 = 2.0 * t12575;
    let tv4rho2sigma23 = t37331 - t37332 + t37333 - t37334 + t37335 - t37336 - t37337 + t7 * (t38519 + t38522) + t38525 - t38526 - t38527 + t38528 + 2.0 * t12435 + t38530;
    (tv4rho2sigma23,)
}
