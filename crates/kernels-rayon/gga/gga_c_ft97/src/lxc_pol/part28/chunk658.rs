//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 658/1189 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk658(t452: f64, t5617: f64, t986: f64, t23311: f64, t23312: f64, t23319: f64, t23321: f64, t23344: f64, t23358: f64, t23360: f64, t26461: f64, t26464: f64, t26468: f64, t26472: f64, t26476: f64, t26480: f64, t28: f64, t446: f64, t89: f64) -> f64 {
    let t26487 = t452 * t986 * t5617;
    let t26490 = -t23311 + t23312 / 9.0_f64 + t23319 / 9.0_f64 + t23321 / 9.0_f64 - t446 * t26461 / 3.0_f64 - t446 * t26464 / 3.0_f64 + 2.0_f64 / 3.0_f64 * t446 * t26468 + 2.0_f64 / 3.0_f64 * t446 * t26472 + t89 * t28 * t26476 / 3.0_f64 - t446 * t26480 / 3.0_f64 - t23344 / 27.0_f64 + t23358 / 9.0_f64 + t23360 / 9.0_f64 - t446 * t26487 / 3.0_f64;
    t26490
}
