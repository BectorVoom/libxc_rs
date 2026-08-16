//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 712/1184 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk712(t28848: f64, t296: f64, t4246: f64, t6365: f64, t840: f64, t28850: f64, t1501: f64, t4129: f64, t871: f64, t25246: f64, t25248: f64, t25252: f64, t25284: f64, t29340: f64, t29342: f64, t29346: f64, t29350: f64, t29354: f64, t29356: f64, t446: f64) -> (f64, f64) {
    let t29359 = t296 * t28848;
    let t29363 = t840 * t4246 * t6365;
    let t29366 = t296 * t28850;
    let t29369 = t1501 * t4129;
    let t29371 = t840 * t871 * t29369;
    let t29374 = -2.0_f64 / 9.0_f64 * t25246 - t25248 / 9.0_f64 + t25252 + t29340 / 9.0_f64 - t446 * t29342 / 3.0_f64 + t446 * t29346 / 3.0_f64 + 2.0_f64 / 3.0_f64 * t446 * t29350 - t25284 / 27.0_f64 + t29354 / 27.0_f64 + 2.0_f64 / 3.0_f64 * t446 * t29356 + 2.0_f64 / 3.0_f64 * t446 * t29359 + t446 * t29363 / 3.0_f64 + 2.0_f64 / 3.0_f64 * t446 * t29366 + t446 * t29371 / 3.0_f64;
    (t29369, t29374)
}
