//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 1020/1067 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk1020<F: Float>(t10270: F, t2345: F, t88252: F, t89: F, t2660: F, t88239: F, t1091: F, t22199: F, t10248: F, t446: F, t22386: F, t3690: F, t10409: F, t3699: F, t2665: F, t70141: F, t83606: F, t83619: F, t89772: F, t89775: F, t89778: F, t89781: F, t89785: F, t89789: F, t89794: F) -> (F, F, F, F, F, F, F, F, F) {
    let t89798 = t89 * t2345 * t10270 * t88252;
    let t89802 = t89 * t2345 * t2660 * t88239;
    let t89805 = t1091 * t22199;
    let t89807 = t446 * t10248 * t89805;
    let t89809 = t3690 * t22386;
    let t89811 = t446 * t10409 * t89809;
    let t89813 = t3699 * t22386;
    let t89815 = t446 * t2665 * t89813;
    let t89818 = 4.0 / 3.0 * t89772 + 8.0 / 3.0 * t89775 - t89778 + 8.0 * t89781 + 8.0 * t89785 + 8.0 * t89789 - 8.0 / 3.0 * t70141 - t89794 / 3.0 - 8.0 * t89798 - 2.0 / 3.0 * t89802 + 4.0 / 9.0 * t83606 - 8.0 * t89807 + 8.0 / 3.0 * t89811 - 8.0 * t89815 + 8.0 / 3.0 * t83619;
    (t89798, t89802, t89805, t89807, t89809, t89811, t89813, t89815, t89818)
}
