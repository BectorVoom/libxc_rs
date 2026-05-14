//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 491/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk491<F: Float>(t106: F, t145: F, t1299: F, t2105: F, t146: F, t692: F, t112: F) -> (F, F, F, F) {
    let t3461 = t106 * t145;
    let t3462 = t2105 * t1299;
    let t3466 = t146 * t692;
    let t3467 = t3466 * t112;
    (t3461, t3462, t3466, t3467)
}
