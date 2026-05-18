//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 301/1189 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk301<F: Float>(t558: F, t925: F, t1969: F, t446: F, t2993: F, t569: F, t18: F, t519: F, t3000: F, t89: F, t1018: F, t375: F) -> (F, F, F, F, F) {
    let t3323 = t925 * t558;
    let t3324 = t1969 * t3323;
    let t3325 = t446 * t3324;
    let t3327 = t569 * t2993;
    let t3328 = t446 * t3327;
    let t3330 = t519 * t18;
    let t3332 = t89 * t3000 * t3330;
    let t3335 = t89 * t375 * t1018;
    (t3323, t3325, t3328, t3332, t3335)
}
