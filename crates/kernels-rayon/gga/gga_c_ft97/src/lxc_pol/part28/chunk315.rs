//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 315/1189 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk315(t1901: f64, t2164: f64, t2195: f64, t3281: f64, t3421: f64, t3426: f64, t3431: f64, t3436: f64, t3442: f64, t3447: f64, t3452: f64, t3457: f64, t3460: f64, t3463: f64, t3467: f64, t3471: f64, t446: f64) -> f64 {
    let t3474 = t2195 / 27.0_f64 + t1901 * t3421 / 9.0_f64 + t1901 * t3426 / 9.0_f64 + t1901 * t3431 / 9.0_f64 + 2.0_f64 / 9.0_f64 * t1901 * t3436 - 2.0_f64 / 27.0_f64 * t1901 * t3442 + t1901 * t3447 / 9.0_f64 + t2164 + 2.0_f64 / 3.0_f64 * t446 * t3452 + t446 * t3457 / 3.0_f64 + t3460 / 27.0_f64 - t446 * t3463 / 9.0_f64 - t446 * t3467 / 9.0_f64 - 2.0_f64 / 9.0_f64 * t3281 * t3471;
    t3474
}
