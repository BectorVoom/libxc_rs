//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 736/1527 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk736<F: Float>(t1268: F, t1458: F, t4028: F, t5450: F, t5456: F, t5493: F, t88: F, t5155: F, t5158: F, t1799: F, t5122: F, t5169: F) -> (F, F, F, F, F) {
    let t6295 = F::cast_from(2.0_f64) * t1268 * t5493 + F::cast_from(4.0_f64) * t1458 * t4028 + F::cast_from(2.0_f64) * t5456 * t88 + t5450;
    let t6299 = F::cast_from(0.11696447245269292414e1_f64) * t5155;
    let t6300 = F::cast_from(0.36622894612013090108e-3_f64) * t5158;
    let t6301 = t5122 * t1799;
    let t6304 = F::cast_from(2.0_f64) * t5169;
    (t6295, t6299, t6300, t6301, t6304)
}
