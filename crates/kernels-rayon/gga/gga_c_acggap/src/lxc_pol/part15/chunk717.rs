//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 717/1278 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk717(t409: f64, t7712: f64, t2082: f64, t7538: f64, t1089: f64, t2080: f64, t429: f64, t2079: f64, t368: f64, t7542: f64, t121: f64, t939: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t7713 = t7712 * t409;
    let t7714 = 0.85748036236139473944e-3_f64 * t7713;
    let t7717 = t7538 * t2082;
    let t7720 = t1089 * t429 * t2080;
    let t7721 = t2079 * t7720;
    let t7722 = 0.21437009059034868486e-3_f64 * t7721;
    let t7724 = t1089 * t368 * t7542;
    let t7725 = t2079 * t7724;
    let t7731 = t939 * t121;
    (t7714, t7717, t7720, t7722, t7724, t7725, t7731)
}
