//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2286/2721 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2286(t15492: f64, t5002: f64, t1174: f64, t18237: f64, t3431: f64, t6187: f64, t698: f64, t1227: f64, t13969: f64, t18341: f64, t18345: f64, t18589: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t65998 = t5002 * t15492;
    let t66001 = t1174 * t3431 * t18237;
    let t66015 = t1174 * t698 * t6187;
    let t66024 = t1227 * t13969 * t18341;
    let t66027 = t1227 * t13969 * t18345;
    let t66052 = t1227 * t13969 * t18589;
    (t65998, t66001, t66015, t66024, t66027, t66052)
}
