//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 16 (v4rho3sigma_4) CSE chunk 1299/1361 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part16_v4rho3sigma_4_chunk1299<F: Float>(t51555: F, t53236: F, t8891: F, t14617: F, t50884: F, t22172: F, t2409: F, t3965: F, t14692: F, t3979: F, t4135: F, t51966: F) -> (F, F, F, F, F) {
    let t54605 = t51555 * t53236 * t8891;
    let t54607 = t50884 * t14617;
    let t54613 = t3965 * t2409 * t22172;
    let t54616 = t3979 * t14692;
    let t54621 = t51966 * t4135;
    (t54605, t54607, t54613, t54616, t54621)
}
