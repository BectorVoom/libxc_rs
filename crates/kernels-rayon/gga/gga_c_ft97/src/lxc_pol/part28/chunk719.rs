//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 719/1189 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk719(t27351: f64, t27364: f64, t27376: f64, t27389: f64, t143: f64, t160: f64, t376: f64, t6687: f64, t89: f64, t144: f64, t26529: f64, t1901: f64, t24003: f64, t24004: f64, t24007: f64, t24054: f64, t27313: f64, t27316: f64, t27320: f64, t27324: f64, t27326: f64, t27330: f64, t27337: f64, t28: f64, t446: f64) -> (f64, f64) {
    let t27391 = t27351 + t27364 + t27376 + t27389;
    let t27393 = t143 * t27391 * t160;
    let t27398 = t89 * t376 * t6687;
    let t27400 = t144 * t26529;
    let t27403 = -t24003 - t446 * t27313 / 3.0_f64 - t446 * t27316 / 3.0_f64 + t24004 / 9.0_f64 - t446 * t27320 / 3.0_f64 - t24007 / 9.0_f64 + t27324 / 9.0_f64 + t1901 * t27326 / 9.0_f64 - 2.0_f64 / 3.0_f64 * t1901 * t27330 - 2.0_f64 * t1901 * t27337 + t89 * t28 * t27393 / 3.0_f64 - t27398 / 9.0_f64 + t24054 + 2.0_f64 / 3.0_f64 * t446 * t27400;
    (t27391, t27403)
}
