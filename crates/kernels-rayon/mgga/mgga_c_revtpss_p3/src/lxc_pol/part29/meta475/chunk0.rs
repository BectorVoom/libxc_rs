//! MGGA_C_REVTPSS lxc pol — lxc_pol part 29 (v4rho3sigma_4) CSE chunk 1748/2049 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1748(t26379: f64, t26702: f64, t3: f64, t2055: f64, t2327: f64, t116: f64, t7373: f64, t670: f64, t2371: f64, t7553: f64, t117: f64, t26153: f64, param_d: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t26703 = t26379 + t26702;
    let t26704 = t3 * t26703;
    let t26716 = param_d * t26703;
    let t26730 = t2327 * t2055;
    let t26733 = t116 * t7373;
    let t26734 = t26733 * t670;
    let t26737 = t7553 * t2371;
    let t26740 = t117 * t26153;
    (t26703, t26704, t26716, t26730, t26733, t26734, t26737, t26740)
}
