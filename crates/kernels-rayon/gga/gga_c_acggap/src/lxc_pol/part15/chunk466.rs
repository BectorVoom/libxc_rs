//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 466/1278 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk466(t2138: f64, t2230: f64, t463: f64, t633: f64, t2147: f64, t157: f64, t2152: f64, t406: f64, t159: f64, t2217: f64, t619: f64, t119: f64, t2143: f64, t2146: f64, t2175: f64, t2178: f64, t2219: f64, t2222: f64, t2228: f64, t464: f64, t616: f64, t639: f64) -> (f64, f64, f64, f64, f64) {
    let t2232 = 0.8673628188205199462e0_f64 * t2138 * t2230;
    let t2235 = t633 * t463;
    let t2236 = t2147 * t2235;
    let t2241 = t2152 * t633 * t406 * t157;
    let t2245 = t619 * t159 * t2217;
    let t2248 = t2175 - t2178 + 0.65854491829355115987e0_f64 * t119 * t2219 - 0.65854491829355115987e0_f64 * t2222 * t464 - t2228 + t2232 - 0.4336814094102599731e0_f64 * t2143 * t639 + 0.8673628188205199462e0_f64 * t2146 * t2236 + 0.4336814094102599731e0_f64 * t2146 * t2241 - 0.4336814094102599731e0_f64 * t616 * t2245;
    (t2232, t2236, t2241, t2245, t2248)
}
