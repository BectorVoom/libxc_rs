//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 559/1395 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk559(t198: f64, t207: f64, t2111: f64, t2114: f64, t2115: f64, t2116: f64, t2133: f64, t2224: f64, t2281: f64, t2285: f64, t2336: f64, t2340: f64, t2343: f64, t2351: f64, t2428: f64, t740: f64, t823: f64) -> f64 {
    let t2432 = t198 * t207 * t2428 * t823 + 6.0_f64 * t198 * t2115 * t2116 + 3.0_f64 * t198 * t2133 * t740 + t2111 + t2114 + t2224 - t2281 - t2285 + t2336 + t2340 - t2343 + t2351;
    t2432
}
