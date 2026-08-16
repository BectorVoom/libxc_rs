//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 740/1173 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk740(t10000: f64, t10004: f64, t10009: f64, t10012: f64, t10015: f64, t10020: f64, t1901: f64, t193: f64, t446: f64, t89: f64, t9845: f64, t9850: f64, t9855: f64, t9976: f64, t9982: f64, t9985: f64, t9989: f64, t9993: f64, t9997: f64) -> f64 {
    let t10022 = 2.0_f64 * t446 * t9845 + t1901 * t9850 / 3.0_f64 + 2.0_f64 / 3.0_f64 * t1901 * t9855 + t89 * t193 * t9976 / 3.0_f64 - t9982 + t1901 * t9985 / 3.0_f64 - t446 * t9989 / 3.0_f64 - 2.0_f64 / 9.0_f64 * t446 * t9993 - t9997 / 3.0_f64 + 4.0_f64 / 9.0_f64 * t10000 + 2.0_f64 * t446 * t10004 - 2.0_f64 / 3.0_f64 * t1901 * t10009 - 2.0_f64 / 9.0_f64 * t10012 - 2.0_f64 / 9.0_f64 * t1901 * t10015 + t446 * t10020;
    t10022
}
