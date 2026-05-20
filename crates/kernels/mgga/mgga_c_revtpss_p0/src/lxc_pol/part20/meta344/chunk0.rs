//! MGGA_C_REVTPSS lxc pol — lxc_pol part 20 (v4rho4_0) CSE chunk 1271/1798 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1271<F: Float>(t16558: F, t342: F, t12050: F, t3154: F, t3151: F, t12046: F, t378: F, t357: F, t379: F, t994: F, t1214: F, t5333: F) -> (F, F, F, F, F, F, F) {
    let t16559 = t342 * t16558;
    let t16560 = t12050 * t3154;
    let t16561 = t16560 * t3151;
    let t16565 = t12046 * t378;
    let t16566 = t342 * t16565;
    let t16568 = t12050 * t3151 * t357;
    let t16603 = t994 * t379;
    let t16696 = t5333 * t1214;
    (t16559, t16561, t16565, t16566, t16568, t16603, t16696)
}
