//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 28 (v4rho3sigma_4) CSE chunk 1946/2041 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1946<F: Float>(t16235: F, t91361: F, t5303: F, t80820: F, t16356: F, t6916: F, t16018: F, t1998: F, t236: F, t6926: F, t1339: F, t54153: F, t550: F, t6936: F) -> (F, F, F, F, F) {
    let t91362 = t91361 * t16235;
    let t91364 = t80820 * t5303;
    let t91366 = t6916 * t16356;
    let t91370 = t6926 * t1998 * t236 * t16018;
    let t91374 = t6936 * t1339 * t54153 * t550;
    (t91362, t91364, t91366, t91370, t91374)
}
