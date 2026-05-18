//! GGA_C_GAPLOC lxc pol — lxc_pol part 18 (v4rho2sigma2_1) CSE chunk 1123/1436 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part18_v4rho2sigma2_1_chunk1123<F: Float>(t2312: F, t9087: F, t20065: F, t2321: F, t9074: F, t1358: F, t9075: F, t2300: F, t6295: F, t6525: F, t2317: F, t6541: F) -> (F, F, F, F, F) {
    let t29852 = F::new(0.47425011059460249332e-2) * t2312 * t9087;
    let t29860 = F::new(0.23712505529730124666e-2) * t9074 * t20065 * t2321;
    let t29862 = F::new(0.63233348079280332442e-2) * t1358 * t9075;
    let t29865 = F::new(0.23712505529730124666e-2) * t6525 * t2300 * t6295;
    let t29868 = F::new(0.47425011059460249332e-2) * t6525 * t6541 * t2317;
    (t29852, t29860, t29862, t29865, t29868)
}
