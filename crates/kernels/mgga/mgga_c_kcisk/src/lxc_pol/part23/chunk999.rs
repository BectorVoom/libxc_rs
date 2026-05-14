//! MGGA_C_KCISK lxc pol — lxc_pol part 23 (v4rho3sigma_3) CSE chunk 999/1447 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part23_v4rho3sigma_3_chunk999<F: Float>(t3984: F, t6157: F, t20025: F, t5675: F, t3937: F, t403: F, t4065: F) -> (F, F, F, F) {
    let t20226 = 0.35981577432354634426e-1 * t6157 * t3984;
    let t20229 = t5675 * t20025;
    let t20230 = t3937 * t20229;
    let t20233 = t4065 * t403;
    (t20226, t20229, t20230, t20233)
}
