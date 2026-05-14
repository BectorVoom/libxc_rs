//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 640/1030 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk640<F: Float>(t147: F, t9478: F, t2252: F, t342: F, t657: F, t173: F, t703: F) -> (F, F, F) {
    let t148 = 10000000.0 <= t147;
    let t9479 = piecewise3(t148, 0.0, t9478);
    let t9482 = t342 * t2252 * t657 / 18.0;
    let t9483 = t173 * t703;
    (t9479, t9482, t9483)
}
