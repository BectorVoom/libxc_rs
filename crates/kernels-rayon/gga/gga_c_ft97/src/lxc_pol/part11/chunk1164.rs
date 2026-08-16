//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 1164/1173 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk1164(t1526: f64, t2640: f64, t42262: f64, t2666: f64, t9483: f64, t10227: f64, t10215: f64, t13598: f64, t294: f64, t9577: f64, t10223: f64, t10214: f64, t10238: f64, t10253: f64, t15567: f64, t18961: f64, t18968: f64, t2320: f64, t3806: f64, t9571: f64, t9583: f64, t9592: f64) -> f64 {
    let t44663 = t1526 * t42262 * t2640;
    let t44666 = t1526 * t9483 * t2666;
    let t44669 = t1526 * t9483 * t10227;
    let t44672 = t1526 * t13598 * t10215;
    let t44674 = t294 * t9577;
    let t44683 = t1526 * t9483 * t10223;
    let t44685 = 2.0_f64 * t10238 + t1526 * t2320 * t10253 / 2.0_f64 - t1526 * t2320 * t10214 * t9571 / 2.0_f64 + t15567 * t18968 * t9592 / 2.0_f64 + t44663 / 18.0_f64 - t44666 / 6.0_f64 - t44669 / 12.0_f64 - t44672 / 9.0_f64 + 2.0_f64 / 3.0_f64 * t1526 * t3806 * t44674 * t9571 - t15567 * t18961 * t9583 / 3.0_f64 + t44683 / 6.0_f64;
    t44685
}
