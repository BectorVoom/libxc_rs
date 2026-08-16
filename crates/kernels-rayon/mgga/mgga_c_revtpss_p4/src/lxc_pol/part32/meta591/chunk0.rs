//! MGGA_C_REVTPSS lxc pol — lxc_pol part 32 (v4rho3sigma_7) CSE chunk 1922/2056 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1922(t103181: f64, t28313: f64, t93317: f64, t4534: f64, t689: f64, t7384: f64, t213: f64, t28340: f64, t26544: f64, t27213: f64, t14983: f64, t26497: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t103182 = t28313 * t103181;
    let t103184 = 0.15421710918628844644e0_f64 * t93317 * t103182;
    let t103196 = 0.10975748638225852664e-1_f64 * t689 * t7384 * t4534;
    let t103212 = t213 * t28340;
    let t103216 = 0.14456046980341999104e-1_f64 * t27213 * t26544;
    let t103219 = 0.19514881078765566038e-1_f64 * t26497 * t14983;
    (t103182, t103184, t103196, t103212, t103216, t103219)
}
