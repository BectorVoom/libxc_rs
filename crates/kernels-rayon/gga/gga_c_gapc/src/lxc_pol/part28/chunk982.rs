//! GGA_C_GAPC lxc pol — lxc_pol part 28 (v4rho2sigma2_7) CSE chunk 982/1429 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part28_v4rho2sigma2_7_chunk982(t11380: f64, t11381: f64, t2993: f64, t3708: f64, t9256: f64, t1453: f64, t435: f64) -> (f64, f64, f64, f64) {
    let t11382 = t11380 * t11381;
    let t11384 = t2993 * t3708;
    let t11385 = t11384 * t9256;
    let t11387 = t435 * t1453;
    (t11382, t11384, t11385, t11387)
}
