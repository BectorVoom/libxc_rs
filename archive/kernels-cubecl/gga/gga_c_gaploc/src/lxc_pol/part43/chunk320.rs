//! GGA_C_GAPLOC lxc pol — lxc_pol part 43 (v4rhosigma3_8) CSE chunk 320/1072 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part43_v4rhosigma3_8_chunk320<F: Float>(t2571: F, t701: F, t1901: F, t550: F, t1843: F, t169: F, t2101: F) -> (F, F, F, F, F) {
    let t2572 = t2571 * t701;
    let t2573 = t1901 * t2572;
    let t2576 = t550 * t2571;
    let t2577 = t1843 * t2576;
    let t2580 = t2101 * t169;
    (t2572, t2573, t2576, t2577, t2580)
}
