//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1266/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk1266<F: Float>(t241: F, t59258: F, t59367: F, t59379: F, t59428: F, t59191: F, t59193: F, t59196: F, t59199: F, t59202: F, t59205: F, t59209: F, t59212: F, t59214: F, t59218: F, t59220: F) -> (F, F) {
    let t59431 = t241 * (t59258 + t59367 + t59379 + t59428);
    let t59432 = -t59191 + t59193 - t59196 - t59199 + t59202 + t59205 + t59209 + t59212 - t59214 - t59218 - t59220 + t59431;
    (t59431, t59432)
}
