//! MGGA_C_REVTPSS lxc pol — lxc_pol part 56 (v4rho2sigma2_11) CSE chunk 1082/1203 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part56_v4rho2sigma2_11_chunk1082(t33529: f64, t3801: f64, t12587: f64, t8951: f64, t44126: f64, t8955: f64, t2172: f64, t7690: f64, t2167: f64, t7700: f64, t1455: f64, t8978: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t125070 = t33529 * t3801;
    let t125074 = t8951 * t12587;
    let t125092 = t8955 * t44126;
    let t125172 = t7690 * t2172;
    let t125174 = t2167 * t7700;
    let t125182 = t1455 * t8978;
    (t125070, t125074, t125092, t125172, t125174, t125182)
}
