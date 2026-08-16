//! MGGA_C_KCIS kxc pol — kxc_pol part 5 (v3rho3_2) CSE chunk 702/1419 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_kxc_pol_part5_v3rho3_2_chunk702(t5127: f64, t5162: f64, t355: f64, t377: f64, t4922: f64, t381: f64, t389: f64, t5025: f64, t1189: f64, t3226: f64, t3436: f64, sigma0: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t5163 = t5127 + t5162;
    let t5164 = t5163 * t355;
    let t5165 = t5164 * sigma0;
    let t5166 = t5165 * t377;
    let t5168 = t4922 * t355;
    let t5169 = t5168 * t381;
    let t5170 = t5169 * t389;
    let t5172 = t5025 * t381;
    let t5173 = t5172 * t1189;
    let t5175 = t3226 * t3436;
    (t5163, t5165, t5166, t5168, t5169, t5170, t5172, t5173, t5175)
}
