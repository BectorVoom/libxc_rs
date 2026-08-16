//! GGA_C_ACGGAP lxc pol — lxc_pol part 13 (v4rho3sigma_5) CSE chunk 1005/1213 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part13_v4rho3sigma_5_chunk1005(t2294: f64, t7630: f64, t31253: f64, t527: f64, t2299: f64, t7610: f64, t33823: f64, t33827: f64, t33831: f64, t33835: f64, t33840: f64, t33842: f64, t33844: f64, t33847: f64, t33852: f64, t33853: f64, t33857: f64, t33860: f64, t33861: f64, t33863: f64) -> f64 {
    let t33865 = t7630 * t2294;
    let t33867 = t31253 * t527;
    let t33869 = t7610 * t2299;
    let t33871 = 0.31448092289604152068e-2_f64 * t33823 - 0.47172138434406228102e-2_f64 * t33827 - 0.62896184579208304136e-3_f64 * t33831 - 0.94344276868812456204e-2_f64 * t33835 - t33840 - t33842 + t33844 + 0.15724046144802076034e-3_f64 * t33847 + t33852 + 0.20965394859736101378e-3_f64 * t33853 + 0.62896184579208304134e-3_f64 * t33857 + t33860 - 35.0_f64 / 432.0_f64 * t33861 - t33863 / 48.0_f64 + 0.25724410870841842184e-2_f64 * t33865 - 0.42874018118069736972e-3_f64 * t33867 + 0.7862023072401038017e-3_f64 * t33869;
    t33871
}
