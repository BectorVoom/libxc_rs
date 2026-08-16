//! MGGA_C_KCIS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 1360/1397 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part26_v4rho3sigma_8_chunk1360(t1385: f64, t1943: f64, t27370: f64, t98144: f64, t102051: f64, t102054: f64, t102057: f64, t102072: f64, t103149: f64, t103199: f64, t12185: f64, t1307: f64, t27369: f64, t28443: f64, t29289: f64, t3984: f64, t7908: f64, t94208: f64, t94287: f64, t98119: f64, t98226: f64) -> (f64, f64, f64) {
    let t103301 = t1943 * t1385;
    let t103303 = t27370 * t98144 * t103301;
    let t103318 = -0.11054629629629629629e-1_f64 * t102051 - 0.46336805555555555557e-3_f64 * t7908 * t12185 * t103199 * t1307 + 0.55652820312500000001e-3_f64 * t27369 * t103303 - 0.58958024691358024689e-2_f64 * t102054 + 0.22109259259259259259e-2_f64 * t102057 + 0.23168402777777777778e-3_f64 * t7908 * t3984 * t103149 * t1307 + 0.6183646701388888889e-4_f64 * t98119 * t28443 + 0.88437037037037037033e-2_f64 * t102072 + t98226 + 0.51485339506172839507e-4_f64 * t94287 - 0.18550940104166666667e-3_f64 * t94208 * t29289;
    (t103301, t103303, t103318)
}
