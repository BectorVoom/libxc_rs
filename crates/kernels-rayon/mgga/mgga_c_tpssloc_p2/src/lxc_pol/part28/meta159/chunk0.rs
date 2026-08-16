//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 28 (v4rho3sigma_4) CSE chunk 803/2041 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk803(t3493: f64, t475: f64, t1214: f64, t248: f64, t3030: f64, t466: f64, t3032: f64, t1208: f64, t476: f64, t478: f64, t3036: f64, t483: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t3494 = t3493 * t475;
    let t3496 = t248 * t1214 * t3494;
    let t3499 = t466 * t3030;
    let t3500 = t3499 * t3032;
    let t3502 = 1.0_f64 / t1208 / t476;
    let t3503 = t3502 * t478;
    let t3504 = t483 * t3036;
    (t3494, t3496, t3499, t3500, t3502, t3503, t3504)
}
