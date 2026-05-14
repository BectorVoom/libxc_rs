//! GGA_C_FT97 lxc pol — lxc_pol part 20 (v4rho3sigma_5) CSE chunk 1057/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part20_v4rho3sigma_5_chunk1057<F: Float>(t108057: F, t24438: F, t6118: F, t27777: F, t96925: F, t108049: F, t446: F, t9770: F, t193: F, t2514: F, t6109: F, t6837: F, t743: F, t12001: F, t27476: F, t1882: F, t27484: F) -> (F, F, F, F, F, F, F) {
    let t108059 = t6118 * t24438 * t108057;
    let t108060 = t96925 * t27777;
    let t108061 = t108060 / 18.0;
    let t108063 = t446 * t9770 * t108049;
    let t108068 = t6109 * t193 * t743 * t6837 * t2514;
    let t108070 = t12001 * t27476;
    let t108072 = t1882 * t27484;
    (t108059, t108060, t108061, t108063, t108068, t108070, t108072)
}
