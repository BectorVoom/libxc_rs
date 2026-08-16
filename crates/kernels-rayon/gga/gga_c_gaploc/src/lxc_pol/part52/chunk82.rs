//! GGA_C_GAPLOC lxc pol — lxc_pol part 52 (v4rhosigma3_17) CSE chunk 82/1013 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part52_v4rhosigma3_17_chunk82(t27: f64, t13: f64, t14: f64, t1: f64, t3: f64, t6: f64, t78: f64) -> (f64, f64, f64, f64, f64) {
    let t338 = t27 * t27;
    let t339 = 1.0_f64 / t338;
    let t340 = t13 * t339;
    let t341 = 1.0_f64 / t14;
    let t342 = t341 * t1;
    let t343 = t3 * t6;
    let t344 = t343 * t78;
    let t345 = t342 * t344;
    (t340, t341, t343, t344, t345)
}
