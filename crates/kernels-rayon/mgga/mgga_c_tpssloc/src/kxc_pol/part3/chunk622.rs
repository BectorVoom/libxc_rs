//! MGGA_C_TPSSLOC kxc pol — kxc_pol part 3 (v3rho3_1) CSE chunk 622/1255 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_kxc_pol_part3_v3rho3_1_chunk622(t248: f64, t3051: f64, t884: f64, t1041: f64, t1044: f64, t2780: f64, t283: f64, t883: f64) -> (f64, f64, f64, f64) {
    let t3053 = t248 * t3051 * t884;
    let t3054 = t1041 * t3053;
    let t3057 = t248 * t1044 * t2780;
    let t3061 = 1.0_f64 / t283 / t883;
    (t3053, t3054, t3057, t3061)
}
