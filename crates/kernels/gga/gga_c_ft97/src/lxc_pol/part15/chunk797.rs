//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 797/1222 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk797<F: Float>(t21655: F, t3885: F, t2606: F, t3892: F, t3891: F, t1091: F, t18729: F, t2599: F, t18740: F, t1131: F, t4917: F, t9803: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t21656 = t3885 * t21655;
    let t21657 = t2606 * t21656;
    let t21660 = t3892 * t21655;
    let t21661 = t3891 * t21660;
    let t21664 = t18729 * t1091;
    let t21665 = t2599 * t21664;
    let t21668 = t18740 * t1091;
    let t21669 = t2606 * t21668;
    let t21672 = t4917 * t1131;
    let t21673 = t3892 * t21672;
    let t21674 = t9803 * t21673;
    (t21656, t21657, t21660, t21661, t21664, t21665, t21668, t21669, t21672, t21673, t21674)
}
