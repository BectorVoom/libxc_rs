//! MGGA_C_REVTPSS lxc pol — lxc_pol part 35 (v4rho3sigma_10) CSE chunk 1182/1234 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part35_v4rho3sigma_10_chunk1182(t1579: f64, t5977: f64, t231: f64, t2723: f64, t1955: f64, t23359: f64, t1558: f64, t6048: f64, t1468: f64, t5962: f64, t23421: f64, t30: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t113285 = t1579 * t5977;
    let t113286 = t113285 * t231;
    let t113295 = t113285 * t2723;
    let t113373 = t1955 * t23359;
    let t113387 = t6048 * t1558 * t231;
    let t113420 = t1468 * t5962;
    let t113424 = t30 * t23421;
    (t113286, t113295, t113373, t113387, t113420, t113424)
}
