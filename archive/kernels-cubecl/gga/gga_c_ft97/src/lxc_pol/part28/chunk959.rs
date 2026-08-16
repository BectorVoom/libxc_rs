//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 959/1189 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk959<F: Float>(t32054: F, t5498: F, t31995: F, t5495: F, t1286: F, t32370: F, t376: F, t1546: F, t32032: F, t2252: F, t342: F, t7155: F) -> (F, F, F, F, F) {
    let t137354 = t32054 * t5498;
    let t137356 = t5495 * t31995;
    let t137363 = t1286 * t376 * t32370;
    let t137376 = t1286 * t1546 * t32032;
    let t137398 = t342 * t2252 * t7155 / F::cast_from(18.0_f64);
    (t137354, t137356, t137363, t137376, t137398)
}
