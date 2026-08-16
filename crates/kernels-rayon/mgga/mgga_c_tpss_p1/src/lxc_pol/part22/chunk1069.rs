//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 1069/1395 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk1069(t11452: f64, t11486: f64, t11518: f64, t11555: f64, t11591: f64, t11628: f64, t11660: f64, t11704: f64, t219: f64, t3988: f64, t1482: f64, t2777: f64, t9067: f64, param_beta: f64) -> (f64, f64, f64, f64) {
    let t11707 = t11452 + t11486 + t11518 + t11555 + t11591 + t11628 + t11660 + t11704;
    let t11708 = param_beta * t11707;
    let t11710 = t3988 * t219;
    let t11722 = t9067 * t1482 * t2777;
    (t11707, t11708, t11710, t11722)
}
