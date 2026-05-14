//! GGA_C_FT97 lxc pol — lxc_pol part 3 (v3rho3_2) CSE chunk 605/887 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part3_v3rho3_2_chunk605<F: Float>(t2344: F, t375: F, t1636: F, t665: F, t670: F, t89: F, t2404: F, t675: F, t2371: F, t683: F, t737: F, t754: F, t2360: F, t761: F, t255: F, t2347: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t9725 = t375 * t2344;
    let t9733 = t1636 * t665;
    let t9735 = t89 * t9733 * t670;
    let t9744 = t2404 * t675;
    let t9770 = t683 * t2371;
    let t9787 = t737 * t754;
    let t9791 = t761 * t2360;
    let t9802 = t2344 * t675;
    let t9803 = t9802 * t255;
    let t9808 = t761 * t2347;
    (t9725, t9733, t9735, t9744, t9770, t9787, t9791, t9802, t9803, t9808)
}
