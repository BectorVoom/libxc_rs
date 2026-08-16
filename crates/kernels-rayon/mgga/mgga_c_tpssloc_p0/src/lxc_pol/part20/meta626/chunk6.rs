//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2262/2712 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2262(t13007: f64, t46843: f64, t131: f64, t205: f64, t41160: f64, t116: f64, t212: f64, t2570: f64, t2585: f64, t4255: f64, t12988: f64, t13005: f64, t221: f64, t2379: f64, t41209: f64, t41212: f64, t41217: f64, t4127: f64, t4128: f64, t46828: f64, t46830: f64, t46836: f64, t46838: f64, t46839: f64, t9458: f64, t9516: f64) -> (f64, f64) {
    let t46844 = t46843 * t13007;
    let t46847 = t205 * t41160 * t131;
    let t46853 = t116 * t212;
    let t46855 = t2585 * t2570 * t46853 * t4255;
    let t46856 = 0.14999999999999999999e-1_f64 * t46855;
    let t46858 = 0.49999999999999999998e-2_f64 * t4127 * t221 * t4128 * t9516 - 0.74999999999999999997e-2_f64 * t46828 - 0.69999999999999999996e-1_f64 * t46830 - 0.59999999999999999997e-1_f64 * t13005 * t221 * t12988 * t2379 - 0.34999999999999999998e-1_f64 * t46836 - 0.59999999999999999997e-1_f64 * t13005 * t46838 * t46839 + 0.13999999999999999999e0_f64 * t46844 + 0.99999999999999999995e-1_f64 * t46847 * t221 * t4128 * t9458 - t46856 + t41209 + t41212 + 0.83333333333333333331e-3_f64 * t41217;
    (t46853, t46858)
}
