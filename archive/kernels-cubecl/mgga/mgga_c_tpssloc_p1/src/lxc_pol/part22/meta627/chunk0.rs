//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2162/2721 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2162<F: Float>(t54047: F, t40167: F, t820: F, t16060: F, t3798: F, t12345: F, t5310: F, t1827: F, t40123: F, t3802: F, t39947: F, t1788: F, t9212: F) -> (F, F, F, F, F, F, F, F) {
    let t54048 = F::cast_from(119.0_f64) / F::cast_from(4608.0_f64) * t54047;
    let t54063 = t40167 * t820;
    let t54124 = t16060 * t3798;
    let t54131 = t12345 * t5310;
    let t54132 = F::cast_from(595.0_f64) / F::cast_from(1152.0_f64) * t54131;
    let t54151 = t40123 * t1827;
    let t54162 = t16060 * t3802;
    let t54198 = t39947 * t1827;
    let t54199 = F::cast_from(119.0_f64) / F::cast_from(4608.0_f64) * t54198;
    let t54312 = t9212 * t1788;
    (t54048, t54063, t54124, t54132, t54151, t54162, t54199, t54312)
}
