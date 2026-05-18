//! GGA_C_GAPLOC lxc pol — lxc_pol part 46 (v4rhosigma3_11) CSE chunk 864/1029 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part46_v4rhosigma3_11_chunk864<F: Float>(t42111: F, t42113: F, t42114: F, t42115: F, t42117: F, t42118: F, t42119: F, t42120: F, t42121: F, t42122: F, t493: F, t1: F, t1415: F, t2413: F, t31730: F) -> (F, F, F) {
    let t42123 = t42111 - t42113 + t42114 - t42115 / F::new(2.0) + t42117 + t42118 - t42119 + t42120 - t42121 - t42122;
    let t42130 = t493 * t42123;
    let t42138 = t1415 * t31730 * t1 * t2413;
    (t42123, t42130, t42138)
}
