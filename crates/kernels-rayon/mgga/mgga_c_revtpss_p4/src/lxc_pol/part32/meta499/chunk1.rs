//! MGGA_C_REVTPSS lxc pol — lxc_pol part 32 (v4rho3sigma_7) CSE chunk 1783/2056 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1783(t28799: f64, t28822: f64, t28861: f64, t28923: f64, t532: f64, t1450: f64, t5627: f64, t9069: f64, t26411: f64, t7900: f64, t28176: f64, t7488: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t28925 = t28799 + t28822 + t28861 + t28923;
    let t28926 = t532 * t28925;
    let t28927 = t28926 * t1450;
    let t28929 = t9069 * t5627;
    let t28932 = t26411 * t7900;
    let t28935 = t7488 * t28176;
    (t28925, t28926, t28927, t28929, t28932, t28935)
}
