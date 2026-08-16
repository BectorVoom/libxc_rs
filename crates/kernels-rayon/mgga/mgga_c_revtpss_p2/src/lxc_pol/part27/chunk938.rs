//! MGGA_C_REVTPSS lxc pol — lxc_pol part 27 (v4rho3sigma_2) CSE chunk 938/1333 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part27_v4rho3sigma_2_chunk938(t1038: f64, t3229: f64, t1036: f64, t1033: f64, t3169: f64, t3173: f64, t3140: f64, t989: f64, t3149: f64, t3160: f64, t2862: f64, t3128: f64) -> (f64, f64, f64, f64, f64) {
    let t11266 = t3229 * t1038;
    let t11267 = t1036 * t11266;
    let t11268 = t1033 * t11267;
    let t11271 = t3169 * t3173;
    let t11273 = t989 * t3140;
    let t11274 = t11273 * t3149;
    let t11277 = t11273 * t3160;
    let t11280 = t3128 * t2862;
    (t11268, t11271, t11274, t11277, t11280)
}
