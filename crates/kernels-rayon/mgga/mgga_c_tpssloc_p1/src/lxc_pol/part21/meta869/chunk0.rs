//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 3182/3221 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3182(t15495: f64, t4997: f64, t15492: f64, t5019: f64, t15591: f64, t5002: f64, t1174: f64, t18237: f64, t3431: f64, t6187: f64, t698: f64, t1227: f64, t13969: f64, t18341: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t65992 = t15495 * t4997;
    let t65994 = t5019 * t15492;
    let t65996 = t15591 * t4997;
    let t65998 = t5002 * t15492;
    let t66001 = t1174 * t3431 * t18237;
    let t66015 = t1174 * t698 * t6187;
    let t66024 = t1227 * t13969 * t18341;
    (t65992, t65994, t65996, t65998, t66001, t66015, t66024)
}
