//! GGA_C_FT97 lxc pol — lxc_pol part 20 (v4rho3sigma_5) CSE chunk 1079/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part20_v4rho3sigma_5_chunk1079<F: Float>(t108401: F, t3281: F, t9744: F, t108020: F, t2354: F, t446: F, t108024: F, t108028: F, t108033: F, t108008: F, t9770: F, t108012: F, t108016: F, t108004: F, t1882: F, t27863: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t108403 = t3281 * t9744 * t108401;
    let t108406 = t446 * t2354 * t108020;
    let t108409 = t446 * t9744 * t108024;
    let t108412 = t3281 * t2354 * t108028;
    let t108415 = t446 * t9744 * t108033;
    let t108418 = t446 * t9770 * t108008;
    let t108421 = t3281 * t9770 * t108012;
    let t108424 = t446 * t2354 * t108016;
    let t108427 = t446 * t9770 * t108004;
    let t108429 = t1882 * t27863;
    (t108403, t108406, t108409, t108412, t108415, t108418, t108421, t108424, t108427, t108429)
}
