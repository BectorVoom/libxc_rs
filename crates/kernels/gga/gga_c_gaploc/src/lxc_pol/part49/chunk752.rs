//! GGA_C_GAPLOC lxc pol — lxc_pol part 49 (v4rhosigma3_14) CSE chunk 752/1028 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part49_v4rhosigma3_14_chunk752<F: Float>(t12161: F, t325: F, t1858: F, t3720: F, t38907: F, t739: F, t2089: F, t7290: F, t321: F, t107: F, t787: F, t12251: F, t1980: F, t296: F, t1: F, t2021: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t38974 = t325 * t12161;
    let t39002 = t1858 * t3720;
    let t39022 = t739 * t38907;
    let t39027 = t2089 * t12161;
    let t39040 = t7290 * t38907;
    let t39048 = t321 * t3720;
    let t39050 = t787 * t39048 * t107;
    let t39118 = t1980 * t12251;
    let t39121 = t296 * t12161;
    let t39123 = t787 * t39121 * t1;
    let t39145 = t39048 * t1;
    let t39146 = t2021 * t39145;
    (t38974, t39002, t39022, t39027, t39040, t39048, t39050, t39118, t39123, t39145, t39146)
}
