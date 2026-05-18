//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 654/1451 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk654<F: Float>(t3104: F, t5328: F, t3109: F, t3133: F, t3151: F, t4570: F, t894: F) -> (F, F, F, F, F) {
    let t5329 = t3104 * t5328;
    let t5330 = t5329 * t3109;
    let t5333 = t5329 * t3133;
    let t5336 = t3151 * t4570;
    let t5337 = t894 * t5336;
    (t5329, t5330, t5333, t5336, t5337)
}
