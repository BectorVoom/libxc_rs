//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1037/1414 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk1037<F: Float>(t22626: F, t22290: F, t22293: F, t22296: F, t22300: F, t22303: F, t22306: F, t22340: F, t22342: F, t22344: F, t22621: F, t22623: F, t22625: F) -> (F, F) {
    let t22627 = F::cast_from(384.0_f64) * t22626;
    let t22628 = -t22290 + t22293 + t22296 - t22300 + t22303 + t22306 + t22340 + t22342 + t22344 + t22621 - t22623 + t22625 - t22627;
    (t22627, t22628)
}
