//! MGGA_C_REVTPSS lxc pol — lxc_pol part 31 (v4rho3sigma_6) CSE chunk 1636/2259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1636(t20823: f64, t5268: f64, t1042: f64, t5265: f64, t5274: f64, t1774: f64, t3362: f64, t4181: f64, t12787: f64, t12916: f64, t6689: f64, t3718: f64) -> (f64, f64, f64, f64) {
    let t20913 = t5268 * t20823;
    let t20914 = t1042 * t20913;
    let t20917 = t5274 * t5265;
    let t20921 = t1774 * t3362;
    let t20922 = t20921 * t4181;
    let t20923 = t12787 * t20922;
    let t20926 = t12916 * t6689;
    let t20927 = t3718 * t20926;
    (t20914, t20917, t20923, t20927)
}
