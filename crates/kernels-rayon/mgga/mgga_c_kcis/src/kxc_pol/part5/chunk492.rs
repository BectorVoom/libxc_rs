//! MGGA_C_KCIS kxc pol — kxc_pol part 5 (v3rho3_2) CSE chunk 492/1419 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_kxc_pol_part5_v3rho3_2_chunk492(t2061: f64, t584: f64, t578: f64, t2011: f64, t555: f64, t583: f64, t2036: f64, t2040: f64, t2044: f64, t2048: f64, t2052: f64, t2056: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t2062 = t2061 * t584;
    let t2063 = t578 * t2062;
    let t2065 = t555 * t2011;
    let t2066 = t583 * t2065;
    let t2067 = t578 * t2066;
    let t2069 = t2036 / 16.0_f64 - t2040 / 16.0_f64 - t2044 / 6.0_f64 + t2048 / 24.0_f64 - t2052 / 256.0_f64 + t2056 / 256.0_f64 + t2063 / 48.0_f64 - t2067 / 192.0_f64;
    (t2062, t2063, t2065, t2066, t2067, t2069)
}
