//! GGA_C_ACGGAP lxc pol — lxc_pol part 11 (v4rho3sigma_3) CSE chunk 471/1213 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part11_v4rho3sigma_3_chunk471(t119: f64, t1959: f64, t1962: f64, t2124: f64, t2127: f64, t2136: f64, t2142: f64, t2143: f64, t2146: f64, t2149: f64, t2155: f64, t2159: f64, t464: f64, t616: f64, t621: f64) -> f64 {
    let t2162 = t1959 - t1962 + 0.65854491829355115987e0_f64 * t119 * t2124 - 0.65854491829355115987e0_f64 * t2127 * t464 - t2136 + t2142 - 0.4336814094102599731e0_f64 * t2143 * t621 + 0.8673628188205199462e0_f64 * t2146 * t2149 + 0.4336814094102599731e0_f64 * t2146 * t2155 - 0.4336814094102599731e0_f64 * t616 * t2159;
    t2162
}
