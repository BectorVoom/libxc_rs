//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 783/1030 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk783<F: Float>(t3088: F, t37320: F, t419: F, t1725: F, t8093: F, t7705: F, t7789: F, t8106: F, t11262: F, t7807: F, t37269: F, t11269: F, t37311: F, t1748: F, t8130: F, t1739: F) -> (F, F, F, F, F, F, F, F, F) {
    let t37702 = t419 * t3088 * t37320;
    let t37704 = t1725 * t8093;
    let t37707 = t419 * t7705 * t7789;
    let t37709 = t1725 * t8106;
    let t37712 = t419 * t11262 * t7807;
    let t37715 = t419 * t3088 * t37269;
    let t37718 = t419 * t11269 * t37311;
    let t37720 = t8130 * t1748;
    let t37723 = t8130 * t1739;
    (t37702, t37704, t37707, t37709, t37712, t37715, t37718, t37720, t37723)
}
