//! GGA_C_FT97 kxc pol — kxc_pol part 3 (v3rho3_2) CSE chunk 642/887 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_kxc_pol_part3_v3rho3_2_chunk642<F: Float>(t13722: F, t11401: F, t665: F, t3705: F, t89: F, t1132: F, t1636: F, t3718: F, t681: F, t375: F, t3822: F, t1882: F, t3714: F, t3692: F, t3696: F, t3701: F) -> (F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F) {
    let t13723 = 4.0 / 81.0 * t13722;
    let t13730 = t11401 * t665;
    let t13732 = t89 * t13730 * t3705;
    let t13739 = t89 * t1636 * t1132;
    let t13740 = 4.0 / 27.0 * t13739;
    let t13746 = t89 * t681 * t3718;
    let t13747 = 4.0 / 9.0 * t13746;
    let t13753 = t89 * t375 * t3822;
    let t13754 = 2.0 / 9.0 * t13753;
    let t13780 = t1882 * t3714;
    let t13781 = 2.0 / 27.0 * t13780;
    let t13794 = t1882 * t3692;
    let t13795 = 4.0 / 81.0 * t13794;
    let t13809 = t1882 * t3696;
    let t13810 = 2.0 / 27.0 * t13809;
    let t13811 = t1882 * t3701;
    (t13723, t13730, t13732, t13739, t13740, t13746, t13747, t13753, t13754, t13780, t13781, t13794, t13795, t13809, t13810, t13811)
}
