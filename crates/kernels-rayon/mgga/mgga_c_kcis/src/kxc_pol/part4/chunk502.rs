//! MGGA_C_KCIS kxc pol — kxc_pol part 4 (v3rho3_1) CSE chunk 502/1420 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_kxc_pol_part4_v3rho3_1_chunk502(t2318: f64, t2321: f64, t2323: f64, t2327: f64, t2329: f64, t2331: f64, t662: f64, t646: f64, t644: f64, t14: f64, t31: f64, t2310: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t2333 = -0.42198333333333333333e0_f64 * t2318 + 0.84396666666666666666e0_f64 * t2321 + 0.39862222222222222223e0_f64 * t2323 + 0.68258333333333333333e-1_f64 * t2327 + 0.13651666666666666667e0_f64 * t2329 + 0.13692777777777777778e0_f64 * t2331;
    let t2334 = t2333 * t662;
    let t2336 = 1.0_f64 * t646 * t2334;
    let t2337 = t644 * t644;
    let t2338 = 1.0_f64 / t2337;
    let t2339 = t14 * t2338;
    let t2340 = t31 * t31;
    let t2341 = 1.0_f64 / t2340;
    let t2342 = t2310 * t2341;
    (t2333, t2334, t2336, t2337, t2338, t2339, t2340, t2341, t2342)
}
