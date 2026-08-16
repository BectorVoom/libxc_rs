//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 50 (v4rho2sigma2_6) CSE chunk 1230/1294 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part50_v4rho2sigma2_6_chunk1230(t1409: f64, t607: f64, t8307: f64, t8513: f64, t31011: f64, t3966: f64, t32: f64, t2240: f64, t8308: f64, t33114: f64, t645: f64, t113862: f64, t113869: f64, t113874: f64, t113880: f64, t113883: f64, t113888: f64, t119880: f64, t119884: f64, t119888: f64, t119892: f64, t119897: f64, t119902: f64, t119905: f64, t119909: f64, t119913: f64, t119917: f64, t31004: f64, t31010: f64, t31013: f64, t33111: f64, t8304: f64) -> f64 {
    let t119924 = t8513 * t8307 * t607 * t1409;
    let t119928 = t8513 * t31011 * t3966;
    let t119931 = t32 * t607;
    let t119932 = t2240 * t119931;
    let t119933 = t8308 * t1409;
    let t119938 = t8513 * t33114 * t645;
    let t119941 = 5.0_f64 / 6.0_f64 * t113862 * t119880 + 5.0_f64 / 6.0_f64 * t113862 * t119884 - 5.0_f64 / 18.0_f64 * t113869 * t119888 - 5.0_f64 / 18.0_f64 * t113874 * t119892 - 5.0_f64 / 18.0_f64 * t113869 * t119897 - 5.0_f64 / 18.0_f64 * t113874 * t119902 - 5.0_f64 / 36.0_f64 * t119905 * t31013 + 35.0_f64 / 24.0_f64 * t113883 * t119909 - 5.0_f64 / 12.0_f64 * t113888 * t119913 - 5.0_f64 / 12.0_f64 * t31004 * t119917 - 5.0_f64 / 36.0_f64 * t113880 * t33111 - 5.0_f64 / 36.0_f64 * t31010 * t119924 - 5.0_f64 / 36.0_f64 * t31010 * t119928 + 5.0_f64 / 18.0_f64 * t119932 * t8304 * t119933 - 5.0_f64 / 12.0_f64 * t113888 * t119938;
    t119941
}
