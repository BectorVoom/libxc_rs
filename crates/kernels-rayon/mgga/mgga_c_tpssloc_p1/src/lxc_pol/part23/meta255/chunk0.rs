//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 916/1527 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk916(t5392: f64, t9321: f64, t9330: f64, t111: f64, t5449: f64, t5465: f64, t626: f64, t5464: f64, t9365: f64, t5489: f64, t5468: f64, t9384: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t19420 = t9321 * t5392;
    let t19430 = t9330 * t5392;
    let t19451 = t5449 * t111;
    let t19471 = t626 * t5465;
    let t19473 = t9365 * t5464;
    let t19480 = t626 * t5489;
    let t19488 = t9384 * t5468;
    (t19420, t19430, t19451, t19471, t19473, t19480, t19488)
}
