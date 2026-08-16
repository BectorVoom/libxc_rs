//! GGA_C_ACGGAP lxc pol — lxc_pol part 12 (v4rho3sigma_4) CSE chunk 471/1250 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part12_v4rho3sigma_4_chunk471(t119: f64, t2143: f64, t2146: f64, t2175: f64, t2178: f64, t2219: f64, t2222: f64, t2228: f64, t2232: f64, t2236: f64, t2241: f64, t2245: f64, t464: f64, t616: f64, t639: f64) -> f64 {
    let t2248 = t2175 - t2178 + 0.65854491829355115987e0_f64 * t119 * t2219 - 0.65854491829355115987e0_f64 * t2222 * t464 - t2228 + t2232 - 0.4336814094102599731e0_f64 * t2143 * t639 + 0.8673628188205199462e0_f64 * t2146 * t2236 + 0.4336814094102599731e0_f64 * t2146 * t2241 - 0.4336814094102599731e0_f64 * t616 * t2245;
    t2248
}
