//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 1007/1041 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk1007<F: Float>(t148403: F, t5899: F, t95344: F, t148408: F, t23667: F, t148412: F, t148417: F, t95340: F, t18: F, t1969: F, t3281: F, t32979: F, t34853: F, t446: F, t558: F, t9432: F) -> (F, F, F, F, F, F) {
    let t148640 = t5899 * t95344 * t148403;
    let t148643 = t5899 * t23667 * t148408;
    let t148646 = t5899 * t23667 * t148412;
    let t148649 = t5899 * t95340 * t148417;
    let t148653 = t3281 * t1969 * t32979 * t18;
    let t148657 = t446 * t9432 * t34853 * t558;
    (t148640, t148643, t148646, t148649, t148653, t148657)
}
