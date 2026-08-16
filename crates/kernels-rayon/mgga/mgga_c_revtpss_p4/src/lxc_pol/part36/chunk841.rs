//! MGGA_C_REVTPSS lxc pol — lxc_pol part 36 (v4rho3sigma_11) CSE chunk 841/1378 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part36_v4rho3sigma_11_chunk841(t11132: f64, t159: f64, t3181: f64, t2851: f64, t631: f64, t45: f64, t992: f64, t338: f64, t378: f64, t1031: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t11133 = 0.46096296296296296297e-1_f64 * t11132;
    let t11142 = t159 * t3181;
    let t11144 = 1.0_f64 / t2851 / t631;
    let t11149 = t2851 * t45;
    let t11150 = 1.0_f64 / t11149;
    let t11198 = t992 * t992;
    let t11199 = 1.0_f64 / t11198;
    let t11200 = t338 * t11199;
    let t11201 = t11200 * t378;
    let t11238 = t1031 * t1031;
    let t11239 = 1.0_f64 / t11238;
    (t11133, t11142, t11144, t11150, t11200, t11201, t11238, t11239)
}
