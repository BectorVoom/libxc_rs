//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 646/1184 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk646(t681: f64, t6909: f64, t89: f64, t1901: f64, t24758: f64, t24815: f64, t28375: f64, t28379: f64, t28382: f64, t28384: f64, t28388: f64, t28392: f64, t28395: f64, t28398: f64, t28401: f64, t28405: f64, t28408: f64, t446: f64) -> f64 {
    let t28411 = t89 * t681 * t6909;
    let t28413 = t24758 / 9.0_f64 - 2.0_f64 / 9.0_f64 * t1901 * t28375 - t1901 * t28379 / 9.0_f64 - t28382 / 9.0_f64 - 2.0_f64 / 9.0_f64 * t28384 - 2.0_f64 / 9.0_f64 * t1901 * t28388 - t446 * t28392 / 3.0_f64 - t24815 + t1901 * t28395 / 9.0_f64 + t1901 * t28398 / 9.0_f64 + 2.0_f64 / 9.0_f64 * t1901 * t28401 - 2.0_f64 / 27.0_f64 * t1901 * t28405 - t28408 / 27.0_f64 - t28411 / 9.0_f64;
    t28413
}
