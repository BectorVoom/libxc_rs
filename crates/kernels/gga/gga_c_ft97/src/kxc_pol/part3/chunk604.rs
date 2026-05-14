//! GGA_C_FT97 kxc pol — kxc_pol part 3 (v3rho3_2) CSE chunk 604/887 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_kxc_pol_part3_v3rho3_2_chunk604<F: Float>(t228: F, t231: F, t9634: F, t626: F, t705: F, t701: F, t191: F, t2360: F, t693: F, t10: F, t242: F, t3050: F, t1636: F, t714: F, t89: F, t7514: F) -> (F, F, F, F, F, F, F, F, F) {
    let t9636 = t228 * t9634 * t231;
    let t9637 = 0.70937342644032921812e-2 * t9636;
    let t9638 = t626 * t705;
    let t9639 = t701 * t9638;
    let t9651 = 1.0 / t191 / t2360;
    let t9680 = t693 * t693;
    let t9681 = 1.0 / t9680;
    let t9698 = t10 * t3050 * t242;
    let t9699 = 14.0 / 81.0 * t9698;
    let t9701 = t89 * t1636 * t714;
    let t9707 = t191 * t7514;
    (t9636, t9637, t9639, t9651, t9681, t9698, t9699, t9701, t9707)
}
