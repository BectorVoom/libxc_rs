//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1149/1272 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk1149<F: Float>(t2554: F, t7324: F, t2367: F, t7245: F, t999: F, t10127: F, t2438: F, t24721: F, t24723: F, t24955: F, t24957: F, t24960: F, t24964: F, t24968: F, t24975: F, t8273: F) -> (F,) {
    let t26091 = t2554 * t7324;
    let t26095 = t999 * t2367 * t7245;
    let t26099 = -200.0 / 3.0 * t26091 * t2438 + 2.0 / 9.0 * t26095 - t24721 - 64.0 / 9.0 * t10127 * t8273 - t24723 + t24975 - t24955 + t24957 - t24960 + t24964 + t24968;
    (t26099,)
}
