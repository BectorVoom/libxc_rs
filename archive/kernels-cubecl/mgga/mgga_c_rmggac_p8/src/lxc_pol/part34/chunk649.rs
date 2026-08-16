//! MGGA_C_RMGGAC lxc pol — lxc_pol part 34 (v4rho2sigma2_7) CSE chunk 649/1097 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part34_v4rho2sigma2_7_chunk649<F: Float>(t838: F, t874: F, t107: F, t1539: F, t1454: F, t201: F, t837: F, t235: F, t325: F, t6477: F, t875: F, t899: F) -> (F, F, F, F, F, F, F) {
    let t27176 = t838 * t874;
    let t28317 = t1539 * t107;
    let t29122 = t1454 * t201;
    let t29837 = t837 * t874;
    let t29838 = t235 * t29837;
    let t30080 = t6477 * t325;
    let t30204 = t899 * t875;
    (t27176, t28317, t29122, t29837, t29838, t30080, t30204)
}
