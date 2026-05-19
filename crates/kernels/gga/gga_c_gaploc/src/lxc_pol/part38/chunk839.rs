//! GGA_C_GAPLOC lxc pol — lxc_pol part 38 (v4rhosigma3_3) CSE chunk 839/1003 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part38_v4rhosigma3_3_chunk839<F: Float>(t42651: F, t2321: F, t38019: F, t9074: F, t42673: F, t1063: F, t35908: F, t894: F, t13304: F, t2312: F, t13296: F, t158: F) -> (F, F, F, F, F, F) {
    let t44413 = F::cast_from(0.28455006635676149599e-1_f64) * t42651;
    let t44415 = t9074 * t38019 * t2321;
    let t44416 = F::cast_from(0.11856252764865062333e-2_f64) * t44415;
    let t44420 = F::cast_from(0.63233348079280332443e-2_f64) * t42673;
    let t44423 = F::cast_from(0.28455006635676149599e-1_f64) * t1063 * t894 * t35908;
    let t44424 = t2312 * t13304;
    let t44425 = F::cast_from(0.11856252764865062333e-2_f64) * t44424;
    let t44426 = t158 * t13296;
    (t44413, t44416, t44420, t44423, t44425, t44426)
}
