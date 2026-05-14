//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 917/1030 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk917<F: Float>(t13598: F, t701: F, t9583: F, t173: F, t9666: F, t9483: F, t9592: F, t209: F, t3626: F, t228: F, t231: F, t39370: F, t420: F, t704: F, t2248: F, t705: F) -> (F, F, F, F, F, F, F) {
    let t41502 = t701 * t13598 * t9583;
    let t41505 = t701 * t173 * t9666;
    let t41508 = t701 * t9483 * t9592;
    let t41510 = t209 * t3626;
    let t41512 = t228 * t41510 * t231;
    let t41513 = 0.18916624705075445817e-1 * t41512;
    let t41516 = t701 * t420 * t704 * t39370;
    let t41519 = t701 * t2248 * t705;
    (t41502, t41505, t41508, t41512, t41513, t41516, t41519)
}
