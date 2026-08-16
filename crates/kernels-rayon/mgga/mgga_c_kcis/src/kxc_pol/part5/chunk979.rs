//! MGGA_C_KCIS kxc pol — kxc_pol part 5 (v3rho3_2) CSE chunk 979/1419 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_kxc_pol_part5_v3rho3_2_chunk979(t3753: f64, t530: f64, t174: f64, t1331: f64, t2331: f64, t251: f64, t3977: f64, t11407: f64, t250: f64, t3106: f64, t461: f64, t453: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t11418 = 1.0_f64 / t3753 / t530;
    let t11425 = 1.0_f64 / t3753 / t174;
    let t11455 = t2331 * t1331;
    let t11462 = t251 * t3977;
    let t11479 = 0.93932222222222222223e0_f64 * t11407;
    let t11481 = t250 * t3106 * t461;
    let t11482 = 0.36793333333333333333e0_f64 * t11481;
    let t11491 = 1.0_f64/pow_3_2(t453);
    (t11418, t11425, t11455, t11462, t11479, t11481, t11482, t11491)
}
