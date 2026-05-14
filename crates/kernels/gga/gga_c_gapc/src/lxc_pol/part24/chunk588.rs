//! GGA_C_GAPC lxc pol — lxc_pol part 24 (v4rho2sigma2_3) CSE chunk 588/1133 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part24_v4rho2sigma2_3_chunk588<F: Float>(t3732: F, t3734: F, t268: F, t3649: F, t2208: F, t2212: F, t772: F) -> (F, F, F, F) {
    let t3735 = t3732 * t3734;
    let t3737 = t3649 * t268;
    let t3738 = t3737 * t2208;
    let t3739 = t772 * t2212;
    (t3735, t3737, t3738, t3739)
}
