//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 946/1124 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk946<F: Float>(t32003: F, t38052: F, t4210: F, t8065: F, t8998: F, t2400: F, t30005: F, t880: F, t9380: F, t2138: F, t2147: F, t322: F, t9413: F, t524: F, t8306: F) -> (F, F, F, F, F, F) {
    let t38055 = 0.34694512752820797848e1 * t32003 * t38052 * t4210;
    let t38065 = t8998 * t8065;
    let t38073 = t30005 * t2400;
    let t38077 = t9380 * t880;
    let t38085 = 0.34694512752820797848e1 * t2138 * t2147 * t9413 * t322;
    let t38086 = t8306 * t524;
    (t38055, t38065, t38073, t38077, t38085, t38086)
}
