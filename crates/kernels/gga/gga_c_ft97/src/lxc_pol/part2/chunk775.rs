//! GGA_C_FT97 lxc pol — lxc_pol part 2 (v3rho3_1) CSE chunk 775/869 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part2_v3rho3_1_chunk775<F: Float>(t11401: F, t665: F, t3705: F, t89: F, t668: F, t737: F, t2999: F, t1132: F, t1636: F, t2459: F, t3717: F, t193: F, t3718: F, t681: F, t13672: F, t676: F) -> (F, F, F, F, F, F, F, F, F) {
    let t13730 = t11401 * t665;
    let t13732 = t89 * t13730 * t3705;
    let t13734 = t737 * t668;
    let t13736 = t89 * t2999 * t13734;
    let t13739 = t89 * t1636 * t1132;
    let t13740 = 4.0 / 27.0 * t13739;
    let t13741 = t3717 * t2459;
    let t13743 = t89 * t193 * t13741;
    let t13746 = t89 * t681 * t3718;
    let t13747 = 4.0 / 9.0 * t13746;
    let t13748 = t676 * t13672;
    (t13730, t13732, t13736, t13739, t13740, t13743, t13746, t13747, t13748)
}
