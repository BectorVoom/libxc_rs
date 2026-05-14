//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 670/1030 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk670<F: Float>(t2594: F, t9583: F, t446: F, t2413: F, t713: F, t2354: F, t2459: F, t684: F, t1882: F, t2356: F, t2336: F, t2362: F, t89: F, t2371: F, t683: F, t2373: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
    let t9754 = t2594 * t9583;
    let t9755 = t446 * t9754;
    let t9757 = t2413 * t713;
    let t9758 = t2354 * t9757;
    let t9759 = t446 * t9758;
    let t9761 = t684 * t2459;
    let t9762 = t2354 * t9761;
    let t9763 = t446 * t9762;
    let t9765 = t1882 * t2356;
    let t9768 = t89 * t2336 * t2362;
    let t9770 = t683 * t2371;
    let t9771 = t684 * t2373;
    (t9754, t9755, t9757, t9758, t9759, t9761, t9762, t9763, t9765, t9768, t9770, t9771)
}
