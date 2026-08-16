//! MGGA_C_REVTPSS lxc pol — lxc_pol part 29 (v4rho3sigma_4) CSE chunk 2008/2049 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk2008(t213: f64, t28340: f64, t26544: f64, t27213: f64, t14983: f64, t26497: f64, t14485: f64, t4481: f64, t95743: f64, t10073: f64, t25402: f64, t7056: f64, t7997: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t103212 = t213 * t28340;
    let t103216 = 0.14456046980341999104e-1_f64 * t27213 * t26544;
    let t103219 = 0.19514881078765566038e-1_f64 * t26497 * t14983;
    let t103220 = t26497 * t14485;
    let t103224 = 0.19514881078765566038e-1_f64 * t95743 * t4481;
    let t103234 = t10073 * t7056 * t25402 * t7997;
    (t103212, t103216, t103219, t103220, t103224, t103234)
}
