//! MGGA_C_REVTPSS lxc pol — lxc_pol part 51 (v4rho2sigma2_6) CSE chunk 882/1200 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part51_v4rho2sigma2_6_chunk882(t114: f64, t1513: f64, t25823: f64, t665: f64, t25826: f64, t4287: f64, t6998: f64, t25822: f64, t25824: f64) -> f64 {
    let t115 = 1.0_f64 < t114;
    let t28034 = t25823 * t1513;
    let t28036 = t1513 * t665;
    let t28037 = t25826 * t28036;
    let t28039 = t6998 * t4287;
    let t28042 = piecewise3(t115, 0.0_f64, t25822 + t25824 / 3.0_f64 + t28034 / 3.0_f64 + t28037 / 4.0_f64 - t28039 / 8.0_f64);
    t28042
}
