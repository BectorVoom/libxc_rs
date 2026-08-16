//! MGGA_C_R2SCAN lxc pol — lxc_pol part 14 (v4rho3sigma_4) CSE chunk 1192/1276 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part14_v4rho3sigma_4_chunk1192(t3700: f64, t40379: f64, t11465: f64, t11523: f64, t11020: f64, t12033: f64, t10985: f64, t12098: f64, t3275: f64, t10610: f64, t3465: f64, t39279: f64) -> (f64, f64, f64, f64, f64) {
    let t41223 = 3.0_f64 / 2.0_f64 * t40379 * t3700;
    let t41225 = 5.0_f64 / 8.0_f64 * t11523 * t11465;
    let t41227 = t11020 * t12033 / 4.0_f64;
    let t41230 = 5.0_f64 / 8.0_f64 * t3275 * t12098 * t10985;
    let t41233 = 3.0_f64 / 2.0_f64 * t10610 * t3465 * t39279;
    (t41223, t41225, t41227, t41230, t41233)
}
