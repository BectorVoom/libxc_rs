//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 716/1030 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk716<F: Float>(t2787: F, t458: F, t2766: F, t9921: F, t4199: F, t9583: F, t10422: F, t2771: F, t10426: F, t2: F, t7640: F, t10262: F, t192: F, t824: F, t2681: F, t2739: F) -> (F, F, F, F, F, F, F, F) {
    let t10559 = t458 * t2787;
    let t10560 = t2766 * t9921;
    let t10563 = t4199 * t9583;
    let t10566 = t2771 * t10422;
    let t10568 = t2771 * t10426;
    let t10570 = t7640 * t2;
    let t10572 = t192 * t10570 * t10262;
    let t10575 = t2 * t824;
    let t10577 = t2681 * t10575 * t2739;
    (t10559, t10560, t10563, t10566, t10568, t10572, t10575, t10577)
}
