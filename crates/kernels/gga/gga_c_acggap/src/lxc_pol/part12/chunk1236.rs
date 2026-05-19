//! GGA_C_ACGGAP lxc pol — lxc_pol part 12 (v4rho3sigma_4) CSE chunk 1236/1250 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part12_v4rho3sigma_4_chunk1236<F: Float>(t38092: F, t7963: F, t7965: F, t4210: F, t7942: F, t2385: F, t323: F, t851: F, t7990: F, t9154: F, t862: F, t865: F) -> (F, F, F, F, F) {
    let t38280 = F::cast_from(0.17347256376410398924e1_f64) * t7963 * t38092 * t7965;
    let t38283 = F::cast_from(0.17347256376410398924e1_f64) * t7942 * t38092 * t4210;
    let t38285 = t851 * t2385 * t323;
    let t38293 = F::cast_from(0.34694512752820797848e1_f64) * t7990 * t9154;
    let t38309 = t862 * t2385 * t865;
    (t38280, t38283, t38285, t38293, t38309)
}
