//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 710/1340 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk710<F: Float>(t5089: F, t555: F, t12: F, t137: F, t139: F, t24: F, t1626: F, t501: F, t572: F, t81: F, t79: F, t127: F) -> (F, F, F, F, F, F, F, F) {
    let t5091 = F::cast_from(0.10389515463408878255e3_f64) * t555 * t5089;
    let t5093 = F::cast_from(1.0_f64) / t137 / t12;
    let t5106 = F::cast_from(1.0_f64) / t139 / t24;
    let t5130 = F::cast_from(12.0_f64) * t501 * t1626;
    let t5135 = t81 * t572;
    let t5136 = F::cast_from(1.0_f64) / t5135;
    let t5137 = t79 * t5136;
    let t5139 = F::cast_from(120.0_f64) * t5137 * t127;
    (t5091, t5093, t5106, t5130, t5135, t5136, t5137, t5139)
}
