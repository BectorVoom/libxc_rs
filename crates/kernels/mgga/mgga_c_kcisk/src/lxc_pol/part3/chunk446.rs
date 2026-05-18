//! MGGA_C_KCISK lxc pol — lxc_pol part 3 (v3rho3_0) CSE chunk 446/1063 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part3_v3rho3_0_chunk446<F: Float>(t1451: F, t3512: F, t1411: F, t142: F, t179: F, t139: F) -> (F, F, F, F) {
    let t3513 = t3512 * t1451;
    let t3514 = t1411 * t3513;
    let t3516 = t179 * t142;
    let t3517 = t139 * t3516;
    (t3513, t3514, t3516, t3517)
}
