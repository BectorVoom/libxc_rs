//! MGGA_C_REVTPSS lxc pol — lxc_pol part 29 (v4rho3sigma_4) CSE chunk 1963/2049 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1963(t102204: f64, t94771: f64, t122: f64, t72: f64, t8085: f64, t25900: f64, t25899: f64, t28894: f64, t94921: f64, t94802: f64, t28814: f64, t689: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t102225 = t94771 * t102204;
    let t102234 = t8085 * t72 * t122;
    let t102235 = t102234 * t25900;
    let t102237 = 0.25702851531048074406e-1_f64 * t25899 * t102235;
    let t102239 = 0.14456046980341999104e-1_f64 * t94921 * t28894;
    let t102241 = 0.25702851531048074406e-1_f64 * t94802 * t28894;
    let t102244 = t28814 * t689;
    (t102225, t102234, t102235, t102237, t102239, t102241, t102244)
}
