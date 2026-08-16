//! GGA_C_GAPC lxc pol — lxc_pol part 21 (v4rho2sigma2_0) CSE chunk 655/1307 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part21_v4rho2sigma2_0_chunk655<F: Float>(t1642: F, t561: F, t116: F, t4978: F, t188: F, t3137: F, t186: F, t424: F) -> (F, F, F, F) {
    let t5252 = t561 * t1642;
    let t5260 = t116 * t4978;
    let t5261 = t3137 * t188;
    let t5285 = t424 * t186;
    (t5252, t5260, t5261, t5285)
}
