//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 986/1222 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk986<F: Float>(t1882: F, t21970: F, t21966: F, t21953: F, t41962: F, t89: F, t22162: F, t375: F, t21959: F, t21950: F, t21993: F, t21979: F) -> (F, F, F, F, F, F, F, F) {
    let t83720 = t1882 * t21970;
    let t83722 = t1882 * t21966;
    let t83728 = t89 * t41962 * t21953;
    let t83770 = t89 * t375 * t22162;
    let t83772 = t1882 * t21959;
    let t83781 = t1882 * t21950;
    let t83789 = t1882 * t21993;
    let t83792 = t89 * t375 * t21979;
    (t83720, t83722, t83728, t83770, t83772, t83781, t83789, t83792)
}
