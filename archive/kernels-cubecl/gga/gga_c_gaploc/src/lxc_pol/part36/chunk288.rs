//! GGA_C_GAPLOC lxc pol — lxc_pol part 36 (v4rhosigma3_1) CSE chunk 288/1029 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part36_v4rhosigma3_1_chunk288<F: Float>(t2334: F, t447: F, t1064: F, t550: F, t1365: F, t1570: F, t169: F) -> (F, F, F, F, F) {
    let t2335 = t2334 * t447;
    let t2336 = t1064 * t2335;
    let t2339 = t550 * t2334;
    let t2340 = t1365 * t2339;
    let t2343 = t1570 * t169;
    (t2335, t2336, t2339, t2340, t2343)
}
