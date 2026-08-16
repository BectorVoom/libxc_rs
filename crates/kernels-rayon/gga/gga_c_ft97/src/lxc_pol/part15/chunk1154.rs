//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 1154/1222 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk1154(t89497: f64, t89513: f64, t89529: f64, t89545: f64, t21639: f64, t3977: f64, t10007: f64, t1091: f64, t1175: f64, t14175: f64, t1901: f64, t193: f64, t21355: f64, t21399: f64, t21499: f64, t241: f64, t242: f64, t2568: f64, t2574: f64, t258: f64, t2594: f64, t42928: f64, t446: f64, t4934: f64, t4969: f64, t5064: f64, t5073: f64, t729: f64, t81697: f64, t81721: f64, t81723: f64, t89: f64) -> (f64, f64, f64) {
    let t89547 = t89497 + t89513 + t89529 + t89545;
    let t89565 = t3977 * t21639;
    let t89573 = t42928 - 4.0_f64 / 3.0_f64 * t446 * t729 * t1175 * t21399 - 8.0_f64 / 3.0_f64 * t1901 * t14175 * t21499 * t1091 + t89 * t193 * t241 * t89547 * t258 / 3.0_f64 - 8.0_f64 / 3.0_f64 * t81697 - 8.0_f64 / 3.0_f64 * t81721 - 8.0_f64 / 3.0_f64 * t81723 + 8.0_f64 * t446 * t2574 * t2568 * t4934 * t5064 + 16.0_f64 / 9.0_f64 * t446 * t2594 * t1175 * t21355 - 4.0_f64 / 3.0_f64 * t446 * t242 * t89565 + 8.0_f64 / 3.0_f64 * t1901 * t10007 * t4969 * t5073;
    (t89547, t89565, t89573)
}
