//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 569/1184 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk569(t1501: f64, t2842: f64, t6362: f64, t8392: f64, t1882: f64, t6371: f64, t6300: f64, t6349: f64, t681: f64, t89: f64, t6304: f64, t25035: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t25271 = t2842 * t1501;
    let t25284 = t8392 * t6362;
    let t25298 = t1882 * t6371;
    let t25312 = t1882 * t6300;
    let t25315 = t89 * t681 * t6349;
    let t25317 = t1882 * t6304;
    let t25343 = 2.0_f64 / 27.0_f64 * t25035;
    (t25271, t25284, t25298, t25312, t25315, t25317, t25343)
}
