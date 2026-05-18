//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1209/1414 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk1209<F: Float>(t241: F, t24748: F, t24887: F, t24917: F, t24972: F, t24708: F, t24712: F, t24715: F, t24718: F, t24721: F, t24723: F, t24955: F, t24957: F, t24960: F, t24964: F, t24968: F) -> (F, F) {
    let t24975 = t241 * (t24748 + t24887 + t24917 + t24972);
    let t24976 = t24708 + t24712 - t24715 - t24718 - t24721 - t24723 + t24975 - t24955 + t24957 - t24960 + t24964 + t24968;
    (t24975, t24976)
}
