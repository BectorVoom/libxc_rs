//! MGGA_C_REVTPSS lxc pol — lxc_pol part 35 (v4rho3sigma_10) CSE chunk 1068/1234 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part35_v4rho3sigma_10_chunk1068(t1580: f64, t7384: f64, t689: f64, t213: f64, t7997: f64, t27213: f64, t7407: f64, t2061: f64, t2718: f64, t26497: f64, t4481: f64, t212: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t28390 = t7384 * t1580;
    let t28391 = t689 * t28390;
    let t28394 = t213 * t7997;
    let t28422 = t27213 * t7407;
    let t28425 = t2718 * t2061;
    let t28434 = t26497 * t4481;
    let t28447 = t212 * t7997;
    (t28390, t28391, t28394, t28422, t28425, t28434, t28447)
}
