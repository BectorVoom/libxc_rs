//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 4 (v3rho3_2) CSE chunk 923/1228 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part4_v3rho3_2_chunk923<F: Float>(t3700: F, t570: F, t111: F, t1395: F, t5363: F, t580: F, t1404: F, t1851: F, t584: F, t9212: F, t9214: F, t9216: F) -> (F, F, F, F, F, F, F, F) {
    let t12461 = F::cast_from(1.0_f64) / t3700 / t570;
    let t12524 = t1395 * t111;
    let t12541 = F::cast_from(2.0_f64) * t5363 * t580;
    let t12543 = F::cast_from(2.0_f64) * t1851 * t1404;
    let t12560 = F::cast_from(0.348e1_f64) * t584;
    let t12561 = F::cast_from(0.156e1_f64) * t9212;
    let t12562 = F::cast_from(0.312e1_f64) * t9214;
    let t12563 = F::cast_from(0.2312e3_f64) * t9216;
    (t12461, t12524, t12541, t12543, t12560, t12561, t12562, t12563)
}
