//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3484/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3484(t19781: f64, t3091: f64, t43131: f64, t19939: f64, t3127: f64, t3172: f64, t11262: f64, t3161: f64, t6311: f64, t11274: f64, t20029: f64, t11656: f64, t19920: f64) -> (f64, f64, f64, f64, f64) {
    let t65567 = t3091 * t43131 * t19781;
    let t65570 = t3127 * t3172 * t19939;
    let t65581 = t3161 * t11262 * t6311;
    let t65585 = t11274 * t20029;
    let t65589 = t11656 * t19920;
    (t65567, t65570, t65581, t65585, t65589)
}
