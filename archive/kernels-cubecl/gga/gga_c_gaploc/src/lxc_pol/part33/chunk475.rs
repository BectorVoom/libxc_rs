//! GGA_C_GAPLOC lxc pol — lxc_pol part 33 (v4rho2sigma2_16) CSE chunk 475/1464 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part33_v4rho2sigma2_16_chunk475<F: Float>(t106: F, t2405: F, t192: F, t524: F, t529: F, t901: F, t1457: F, t2335: F) -> (F, F, F, F, F) {
    let t2406 = t2405 * t106;
    let t2407 = t2406 * t192;
    let t2410 = t524 * t529;
    let t2411 = t2410 * t901;
    let t2413 = t1457 * t2335;
    (t2406, t2407, t2410, t2411, t2413)
}
