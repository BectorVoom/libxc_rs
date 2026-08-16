//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2745/3221 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2745(t46134: f64, t46137: f64, t4303: f64, t776: f64, t2517: f64, t5520: f64, t40667: f64, t40673: f64, t40680: f64, t2522: f64, t39309: f64, t39312: f64, t39316: f64, t39320: f64, t40679: f64, t4307: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t57891 = 0.96319466275353142155e0_f64 * t46134;
    let t57892 = 0.43374325201206959368e-1_f64 * t46137;
    let t57893 = t776 * t4303;
    let t57897 = t5520 * t2517;
    let t57898 = 0.10389515463408878255e3_f64 * t40667;
    let t57899 = 2.0_f64 * t40673;
    let t57900 = 0.24415263074675393405e-3_f64 * t40680;
    let t57901 = -12.0_f64 * t2522 * t4307 * t57893 - t39309 + t39312 + t39316 + t39320 - t40679 + t57891 + t57892 + t57897 - t57898 + t57899 + t57900;
    (t57891, t57892, t57897, t57898, t57899, t57900, t57901)
}
