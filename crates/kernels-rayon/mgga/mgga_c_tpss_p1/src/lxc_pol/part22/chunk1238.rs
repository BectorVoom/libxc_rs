//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 1238/1395 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk1238(t3432: f64, t578: f64, t1630: f64, t18436: f64, t136: f64, t527: f64, t1693: f64, t215: f64, t4478: f64, t4409: f64, t5716: f64, t18444: f64, t236: f64, t339: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t19417 = t578 * t3432;
    let t19466 = t18436 * t1630;
    let t19468 = t527 * t136;
    let t19469 = t1693 * t19468;
    let t19470 = t215 * t4478;
    let t19471 = t19469 * t19470;
    let t19473 = t5716 * t4409;
    let t19476 = t339 * t18444 * t236;
    (t19417, t19466, t19468, t19469, t19470, t19471, t19473, t19476)
}
