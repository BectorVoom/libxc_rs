//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 662/1030 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk662<F: Float>(t173: F, t2442: F, t701: F, t2447: F, t2451: F, t191: F, t2360: F, t9570: F, t9571: F, t420: F, t2440: F, t9577: F, t3806: F, t9583: F, t2347: F, t703: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t9641 = t173 * t2442;
    let t9642 = t701 * t9641;
    let t9644 = t173 * t2447;
    let t9645 = t701 * t9644;
    let t9647 = t173 * t2451;
    let t9648 = t701 * t9647;
    let t9651 = 1.0 / t191 / t2360;
    let t9652 = t9651 * t9570;
    let t9653 = t9652 * t9571;
    let t9654 = t420 * t9653;
    let t9655 = t701 * t9654;
    let t9657 = t2440 * t9577;
    let t9658 = t9657 * t9571;
    let t9659 = t420 * t9658;
    let t9660 = t701 * t9659;
    let t9662 = t3806 * t9583;
    let t9663 = t701 * t9662;
    let t9665 = t703 * t2347;
    (t9642, t9645, t9648, t9651, t9653, t9655, t9658, t9660, t9663, t9665)
}
