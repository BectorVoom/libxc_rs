//! MGGA_C_RMGGAC lxc pol — lxc_pol part 34 (v4rho2sigma2_7) CSE chunk 1088/1097 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part34_v4rho2sigma2_7_chunk1088(t739: f64, t77871: f64, t72164: f64, t72173: f64, t14581: f64, t8537: f64, t14585: f64, t8672: f64, t1356: f64, t13957: f64, t43974: f64, t7879: f64, t884: f64, t9530: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t78590 = t739 * t77871;
    let t78591 = 0.14967802127329760705e-1_f64 * t78590;
    let t78592 = 0.36021158228745895953e-3_f64 * t72164;
    let t78593 = 0.51240438831339423711e-4_f64 * t72173;
    let t78594 = t14581 * t8537;
    let t78595 = 0.27274661654245341728e-1_f64 * t78594;
    let t78596 = t14585 * t8672;
    let t78597 = 0.36366215538993788971e-1_f64 * t78596;
    let t78602 = 0.11974241701863808564e0_f64 * t1356 * t43974 * t13957;
    let t78605 = 0.11974241701863808564e0_f64 * t884 * t9530 * t7879;
    (t78591, t78592, t78593, t78595, t78597, t78602, t78605)
}
