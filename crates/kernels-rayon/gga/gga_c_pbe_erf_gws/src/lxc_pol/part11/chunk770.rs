//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 770/1302 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk770(t12350: f64, t5003: f64, t1640: f64, t639: f64, t1010: f64, t10848: f64, t7122: f64, t10329: f64, t12339: f64, t1664: f64, t590: f64, t587: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t12563 = t5003 * t12350;
    let t12564 = t1640 * t12563;
    let t12566 = 8.0_f64 / 9.0_f64 * t639 * t12564;
    let t12568 = 4.0_f64 / 15.0_f64 * t10848 * t1010;
    let t12569 = 4.0_f64 / 45.0_f64 * t7122;
    let t12570 = 16.0_f64 / 15.0_f64 * t10329;
    let t12571 = t1664 * t12339;
    let t12572 = t590 * t12571;
    let t12574 = 8.0_f64 / 15.0_f64 * t587 * t12572;
    (t12563, t12564, t12566, t12568, t12569, t12570, t12571, t12572, t12574)
}
