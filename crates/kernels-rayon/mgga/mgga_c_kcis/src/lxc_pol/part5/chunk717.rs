//! MGGA_C_KCIS lxc pol — lxc_pol part 5 (v3rho3_2) CSE chunk 717/1419 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part5_v3rho3_2_chunk717(t174: f64, t236: f64, t5398: f64, t233: f64, t1301: f64, t1881: f64, t1641: f64, t4532: f64, t447: f64, t637: f64, t446: f64, t1640: f64, t1885: f64, zeta_threshold: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t175 = t174 <= zeta_threshold;
    let t5399 = t236 * t5398;
    let t5400 = t233 * t5399;
    let t5402 = t1881 * t1301;
    let t5404 = t1881 * t1641;
    let t5406 = piecewise3(t175, 0.0_f64, -t4532);
    let t5407 = t447 * t5406;
    let t5408 = t5407 * t637;
    let t5409 = t446 * t5408;
    let t5411 = t1885 * t1640;
    (t5400, t5402, t5404, t5407, t5408, t5409, t5411)
}
