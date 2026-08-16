//! GGA_C_FT97 kxc pol — kxc_pol part 3 (v3rho3_2) CSE chunk 672/1032 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_kxc_pol_part3_v3rho3_2_chunk672(t760: f64, t255: f64, t9895: f64, t2492: f64, t754: f64, t9698: f64, t726: f64, t8232: f64, t192: f64, t7514: f64, t2252: f64, t342: f64, t784: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t10050 = t760 * t760;
    let t10051 = 1.0_f64 / t10050;
    let t10052 = t255 * t10051;
    let t10079 = t9895 * t255;
    let t10085 = t2492 * t754;
    let t10119 = 28.0_f64 / 27.0_f64 * t9698;
    let t10134 = t8232 * t726;
    let t10157 = t192 * t7514;
    let t10207 = t342 * t2252 * t784 / 18.0_f64;
    (t10052, t10079, t10085, t10119, t10134, t10157, t10207)
}
