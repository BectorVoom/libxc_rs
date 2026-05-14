//! GGA_C_FT97 lxc pol — lxc_pol part 23 (v4rho3sigma_8) CSE chunk 618/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part23_v4rho3sigma_8_chunk618<F: Float>(t2347: F, t743: F, t26: F, t666: F, t2360: F, t1087: F, t89: F, t9733: F, t11401: F, t665: F, t3705: F, t1132: F, t1636: F, t3718: F, t681: F, t375: F, t3822: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
    let t13683 = t743 * t2347;
    let t13688 = t26 * t666;
    let t13689 = t743 * t2360;
    let t13722 = t89 * t9733 * t1087;
    let t13723 = 4.0 / 81.0 * t13722;
    let t13730 = t11401 * t665;
    let t13732 = t89 * t13730 * t3705;
    let t13739 = t89 * t1636 * t1132;
    let t13740 = 4.0 / 27.0 * t13739;
    let t13746 = t89 * t681 * t3718;
    let t13747 = 4.0 / 9.0 * t13746;
    let t13753 = t89 * t375 * t3822;
    (t13683, t13688, t13689, t13722, t13723, t13730, t13732, t13739, t13740, t13746, t13747, t13753)
}
