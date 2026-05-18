//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 707/1173 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk707<F: Float>(t420: F, t9653: F, t701: F, t2440: F, t9577: F, t9571: F, t3806: F, t9583: F, t2347: F, t703: F, t2320: F, t9592: F) -> (F, F, F, F, F, F, F) {
    let t9654 = t420 * t9653;
    let t9655 = t701 * t9654;
    let t9657 = t2440 * t9577;
    let t9658 = t9657 * t9571;
    let t9659 = t420 * t9658;
    let t9660 = t701 * t9659;
    let t9662 = t3806 * t9583;
    let t9663 = t701 * t9662;
    let t9665 = t703 * t2347;
    let t9666 = t9665 * t9571;
    let t9667 = t420 * t9666;
    let t9668 = t701 * t9667;
    let t9670 = t2320 * t9592;
    (t9655, t9658, t9660, t9663, t9666, t9668, t9670)
}
