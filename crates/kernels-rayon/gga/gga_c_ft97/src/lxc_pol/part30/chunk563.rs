//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 563/1184 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk563(t25035: f64, t1476: f64, t2347: f64, t1882: f64, t6336: f64, t6260: f64, t668: f64, t2691: f64, t6248: f64, t24330: f64, t6242: f64, t6243: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t25036 = 2.0_f64 / 9.0_f64 * t25035;
    let t25037 = t1476 * t2347;
    let t25042 = t1882 * t6336;
    let t25044 = t6260 * t668;
    let t25049 = t2691 * t6248;
    let t25055 = t6242 * t24330 * t6243;
    (t25036, t25037, t25042, t25044, t25049, t25055)
}
