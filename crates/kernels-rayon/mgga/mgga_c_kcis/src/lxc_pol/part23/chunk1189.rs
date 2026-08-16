//! MGGA_C_KCIS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 1189/1323 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part23_v4rho3sigma_5_chunk1189(t3245: f64, t7935: f64, t18210: f64, t2237: f64, t27395: f64, t27402: f64, t16937: f64, t27358: f64, t27369: f64, t10470: f64, t2244: f64, t27339: f64, t94469: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t94539 = t3245 * t7935;
    let t94546 = t2237 * t18210 * t27395;
    let t94554 = t2237 * t18210 * t27402;
    let t94585 = t16937 * t27358;
    let t94586 = t27369 * t94585;
    let t94588 = t10470 * t2244;
    let t94589 = 0.73697530864197530862e-3_f64 * t94588;
    let t94592 = t27339 * t94469;
    (t94539, t94546, t94554, t94585, t94586, t94588, t94589, t94592)
}
