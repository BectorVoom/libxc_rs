//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 1047/1332 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk1047(t3382: f64, t4300: f64, t4304: f64, t3409: f64, t1181: f64, t12991: f64, t3650: f64, t530: f64, t13087: f64, t4273: f64, t4713: f64, t1101: f64, t1165: f64, t1567: f64, t4282: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t18155 = t3382 * t4300;
    let t18157 = t3382 * t4304;
    let t18159 = t3409 * t4304;
    let t18164 = t12991 * t1181 * t530 * t3650;
    let t18166 = t13087 * t4273;
    let t18176 = t3382 * t4713;
    let t18189 = t4282 * t1165 * t1567 * t1101;
    (t18155, t18157, t18159, t18164, t18166, t18176, t18189)
}
