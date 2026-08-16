//! MGGA_C_REVTPSS lxc pol — lxc_pol part 35 (v4rho3sigma_10) CSE chunk 997/1234 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part35_v4rho3sigma_10_chunk997(t5192: f64, t6552: f64, t1188: f64, t24375: f64, t3520: f64, t1196: f64, t1765: f64, t20400: f64, t5197: f64, t6535: f64, t6556: f64, t12485: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t24478 = 0.17544670867903938621e1_f64 * t5192 * t6552;
    let t24480 = t3520 * t24375 * t1188;
    let t24482 = 0.35089341735807877242e1_f64 * t1196 * t24480;
    let t24484 = 0.17544670867903938621e1_f64 * t20400 * t1765;
    let t24488 = t5197 * t6535;
    let t24490 = 0.35089341735807877242e1_f64 * t1196 * t24488;
    let t24492 = 0.51947577317044391276e2_f64 * t5192 * t6556;
    let t24493 = t12485 * t24375;
    (t24478, t24482, t24484, t24490, t24492, t24493)
}
