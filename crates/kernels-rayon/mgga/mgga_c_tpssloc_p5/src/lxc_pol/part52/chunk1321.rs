//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 52 (v4rho2sigma2_8) CSE chunk 1321/1400 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part52_v4rho2sigma2_8_chunk1321(t12725: f64, t8326: f64, t26103: f64, t7467: f64, t26135: f64, t6517: f64, t33211: f64, t6534: f64, t31537: f64, t1873: f64, t96361: f64, t24999: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t120130 = t12725 * t8326;
    let t120131 = 2.0_f64 * t120130;
    let t120132 = t26103 * t7467;
    let t120134 = t6517 * t26135;
    let t120137 = 4.0_f64 * t33211 * t6534;
    let t120140 = 4.0_f64 * t31537 * t7467;
    let t120141 = t96361 * t1873;
    let t120143 = t24999 * t6534;
    (t120131, t120132, t120134, t120137, t120140, t120141, t120143)
}
