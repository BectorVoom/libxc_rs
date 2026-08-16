//! GGA_C_GAPC lxc pol — lxc_pol part 36 (v4rho2sigma2_15) CSE chunk 1177/1328 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part36_v4rho2sigma2_15_chunk1177(t11451: f64, t11518: f64, t20897: f64, t11517: f64, t33490: f64, t34535: f64, t5117: f64, t11438: f64, t19686: f64, t3021: f64, t11442: f64, t19671: f64) -> (f64, f64, f64, f64, f64) {
    let t34654 = t11518 * t11451 * t20897;
    let t34656 = t11517 * t33490;
    let t34658 = t34656 * t34535 * t5117;
    let t34661 = t11438 * t3021 * t19686;
    let t34663 = t19671 * t11442;
    (t34654, t34656, t34658, t34661, t34663)
}
