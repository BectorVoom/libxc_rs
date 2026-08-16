//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 19 (v4rho4_0) CSE chunk 1454/1497 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1454(t44493: f64, t44547: f64, t44600: f64, t44655: f64, t3630: f64, t3493: f64, t491: f64, t11720: f64, t1235: f64, t10469: f64, t1190: f64, t11887: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t44657 = t44493 + t44547 + t44600 + t44655;
    let t44662 = t3630 * t3630;
    let t44668 = t3493 * t3493;
    let t44669 = t491 * t44668;
    let t44673 = t1235 * t11720;
    let t44690 = t1190 * t10469;
    let t44691 = t44690 * t11887;
    (t44657, t44662, t44668, t44669, t44673, t44690, t44691)
}
