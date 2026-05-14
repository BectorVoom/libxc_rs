//! GGA_C_GAPC lxc pol — lxc_pol part 23 (v4rho2sigma2_2) CSE chunk 646/1126 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part23_v4rho2sigma2_2_chunk646<F: Float>(t5: F, t830: F, t2577: F, t869: F, t818: F, t959: F, t314: F, t2299: F, t2546: F, t2763: F, t291: F) -> (F, F, F, F, F, F) {
    let t7549 = t830 * t5;
    let t7553 = t869 * t2577;
    let t7556 = t818 * t959;
    let t7557 = t7556 * t314;
    let t7577 = t2546 * t2299;
    let t7591 = t291 * t2763;
    (t7549, t7553, t7556, t7557, t7577, t7591)
}
