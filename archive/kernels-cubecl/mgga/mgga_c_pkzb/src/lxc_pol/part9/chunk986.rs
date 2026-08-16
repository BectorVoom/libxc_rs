//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 986/1336 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk986<F: Float>(t7857: F, t793: F, t2036: F, t7840: F, t2970: F, t7658: F, t2968: F, t5931: F, t7666: F, t7832: F, t1133: F, t2019: F) -> (F, F, F, F, F, F) {
    let t7858 = t7857 * t793;
    let t7861 = t2036 * t7840;
    let t7864 = t2970 * t7658;
    let t7867 = t5931 * t2968;
    let t7868 = t7832 * t7666;
    let t7871 = t2019 * t1133;
    (t7858, t7861, t7864, t7867, t7868, t7871)
}
