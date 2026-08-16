//! GGA_C_GAPC lxc pol — lxc_pol part 32 (v4rho2sigma2_11) CSE chunk 1246/1311 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part32_v4rho2sigma2_11_chunk1246(t11281: f64, t2011: f64, t13281: f64, t1617: f64, t3659: f64, t11224: f64, t518: f64, t13850: f64, t25042: f64, t190: f64, t467: f64, t13853: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t35375 = t11281 * t2011;
    let t35378 = 24.0_f64 * t13281 * t3659 * t1617;
    let t35379 = t518 * t11224;
    let t35381 = t25042 * t13850;
    let t35382 = t467 * t190;
    let t35384 = t35381 * t35382 * t13853;
    (t35375, t35378, t35379, t35381, t35382, t35384)
}
