//! GGA_C_GAPC lxc pol — lxc_pol part 38 (v4rho2sigma2_17) CSE chunk 442/1307 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part38_v4rho2sigma2_17_chunk442(t2405: f64, t772: f64, t468: f64, t820: f64, t2158: f64, t276: f64, t653: f64, t902: f64, t128: f64, t291: f64) -> (f64, f64, f64, f64) {
    let t2406 = t772 * t2405;
    let t2409 = t468 * t820;
    let t2412 = t2158 * t276;
    let t2413 = t902 * t653;
    let t2414 = t2412 * t2413;
    let t2415 = t128 * t291;
    (t2406, t2409, t2414, t2415)
}
