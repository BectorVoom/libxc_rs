//! GGA_C_GAPLOC lxc pol — lxc_pol part 48 (v4rhosigma3_13) CSE chunk 766/1003 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part48_v4rhosigma3_13_chunk766<F: Float>(t1858: F, t3614: F, t11764: F, t783: F, t1: F, t35659: F, t787: F, t11822: F, t2021: F, t35439: F, t11756: F, t321: F) -> (F, F, F, F, F, F, F, F) {
    let t36390 = t1858 * t3614;
    let t36477 = t11764 * t783;
    let t36506 = t787 * t35659 * t1;
    let t36512 = t2021 * t11822;
    let t36515 = t35439 * t1;
    let t36516 = t2021 * t36515;
    let t36590 = t11756 * t783;
    let t36610 = t321 * t3614;
    (t36390, t36477, t36506, t36512, t36515, t36516, t36590, t36610)
}
