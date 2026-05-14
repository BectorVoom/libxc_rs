//! GGA_C_GAPLOC lxc pol — lxc_pol part 33 (v4rho2sigma2_16) CSE chunk 1016/1294 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part33_v4rho2sigma2_16_chunk1016<F: Float>(t1358: F, t9075: F, t2300: F, t6295: F, t6525: F, t2317: F, t6541: F, t20731: F, t2321: F, t9074: F, t122: F, t2310: F, t481: F) -> (F, F, F, F, F) {
    let t29862 = 0.63233348079280332442e-2 * t1358 * t9075;
    let t29865 = 0.23712505529730124666e-2 * t6525 * t2300 * t6295;
    let t29868 = 0.47425011059460249332e-2 * t6525 * t6541 * t2317;
    let t29871 = 0.47425011059460249332e-2 * t9074 * t20731 * t2321;
    let t29874 = t481 * t2310 * t122;
    (t29862, t29865, t29868, t29871, t29874)
}
