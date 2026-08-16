//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 841/1451 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk841(t16294: f64, t743: f64, t1317: f64, t4741: f64, t201: f64, t5: f64, t1303: f64, t4733: f64, t1256: f64, t12979: f64, t9477: f64, t13062: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t16295 = t743 * t16294;
    let t16298 = t4741 * t1317;
    let t16300 = t5 * t16298 * t201;
    let t16301 = t743 * t16300;
    let t16310 = t4733 * t1303;
    let t16315 = t12979 * t1256;
    let t16318 = 0.35089340384731224426e1_f64 * t9477;
    let t16319 = 0.17544670192365612213e1_f64 * t13062;
    (t16295, t16300, t16301, t16310, t16315, t16318, t16319)
}
