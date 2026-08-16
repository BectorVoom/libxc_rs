//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 19 (v4rho4_0) CSE chunk 1408/1497 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1408(t43808: f64, t43831: f64, t1107: f64, t11223: f64, t699: f64, t11205: f64, t11208: f64, t11219: f64, t136: f64, t43792: f64, t3297: f64, t43796: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t43832 = t43808 + t43831;
    let t43833 = t1107 * t43832;
    let t43835 = t699 * t11223;
    let t43837 = t699 * t11205;
    let t43839 = t699 * t11208;
    let t43842 = t136 * t11219 * t43792;
    let t43845 = t136 * t3297 * t43796;
    (t43832, t43833, t43835, t43837, t43839, t43842, t43845)
}
