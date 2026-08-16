//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 549/1395 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk549(t2348: f64, t2349: f64, t2111: f64, t2114: f64, t2208: f64, t2217: f64, t2220: f64, t2224: f64, t2242: f64, t2244: f64, t2246: f64, t2281: f64, t2285: f64, t2292: f64, t2302: f64, t2310: f64, t2333: f64, t2336: f64, t2340: f64, t2343: f64, t2347: f64) -> (f64, f64) {
    let t2351 = 0.10843581300301739842e-1_f64 * t2348 * t2349;
    let t2352 = -t2208 - t2217 - t2220 + t2224 + t2242 + t2244 + t2246 + t2333 + t2302 + t2310 + t2111 + t2114 + t2336 - t2292 + t2340 - t2281 - t2343 + t2347 - t2285 + t2351;
    (t2351, t2352)
}
