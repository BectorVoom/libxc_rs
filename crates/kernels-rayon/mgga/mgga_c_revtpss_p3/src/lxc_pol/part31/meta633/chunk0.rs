//! MGGA_C_REVTPSS lxc pol — lxc_pol part 31 (v4rho3sigma_6) CSE chunk 2087/2259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2087(t3201: f64, t7801: f64, t1058: f64, t27467: f64, t15775: f64, t7132: f64, t100054: f64, t3299: f64, t4857: f64, t7125: f64, t25495: f64, t4845: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t100272 = t7801 * t3201;
    let t100275 = 0.57165357490759649296e-3_f64 * t27467 * t1058;
    let t100289 = 0.6351706387862183255e-3_f64 * t7132 * t15775;
    let t100302 = t3299 * t100054;
    let t100324 = t4857 * t7125;
    let t100327 = t25495 * t4845;
    (t100272, t100275, t100289, t100302, t100324, t100327)
}
