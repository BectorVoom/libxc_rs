//! GGA_C_GAPLOC lxc pol — lxc_pol part 42 (v4rhosigma3_7) CSE chunk 817/1012 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part42_v4rhosigma3_7_chunk817<F: Float>(t42647: F, t42651: F, t2321: F, t38019: F, t9074: F, t42673: F, t1063: F, t35908: F, t894: F, t13304: F, t2312: F, t13307: F, t6305: F) -> (F, F, F, F, F, F, F) {
    let t44411 = F::new(0.56910013271352299199e-1) * t42647;
    let t44413 = F::new(0.28455006635676149599e-1) * t42651;
    let t44415 = t9074 * t38019 * t2321;
    let t44416 = F::new(0.11856252764865062333e-2) * t44415;
    let t44420 = F::new(0.63233348079280332443e-2) * t42673;
    let t44423 = F::new(0.28455006635676149599e-1) * t1063 * t894 * t35908;
    let t44424 = t2312 * t13304;
    let t44425 = F::new(0.11856252764865062333e-2) * t44424;
    let t44435 = F::new(0.19918504644973304719e0) * t6305 * t13307;
    (t44411, t44413, t44416, t44420, t44423, t44425, t44435)
}
