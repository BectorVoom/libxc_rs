//! GGA_C_FT97 lxc pol — lxc_pol part 23 (v4rho3sigma_8) CSE chunk 581/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part23_v4rho3sigma_8_chunk581<F: Float>(t226: F, t9681: F, t10: F, t242: F, t3050: F, t1636: F, t714: F, t89: F, t191: F, t7514: F, t2344: F, t375: F, t665: F, t670: F, t2404: F, t675: F) -> (F, F, F, F, F, F, F, F, F) {
    let t9682 = t226 * t9681;
    let t9698 = t10 * t3050 * t242;
    let t9699 = 14.0 / 81.0 * t9698;
    let t9701 = t89 * t1636 * t714;
    let t9707 = t191 * t7514;
    let t9725 = t375 * t2344;
    let t9733 = t1636 * t665;
    let t9735 = t89 * t9733 * t670;
    let t9744 = t2404 * t675;
    (t9682, t9698, t9699, t9701, t9707, t9725, t9733, t9735, t9744)
}
