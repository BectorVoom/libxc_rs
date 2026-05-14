//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 1001/1208 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk1001<F: Float>(t2411: F, t465: F, t154: F, t385: F, t386: F, t4932: F, t466: F, t931: F, t53: F, t6404: F, t414: F, t6545: F, t6523: F, t937: F, t6514: F, t6455: F) -> (F, F, F, F, F, F, F, F) {
    let t19155 = t465 * t2411;
    let t19163 = 5.0 / 486.0 * t385 * t154 * t4932 * t386;
    let t19191 = t466 * t931;
    let t19203 = t53 * t6404;
    let t19227 = 1.0 / t6545 / t414;
    let t19271 = t6523 * t937;
    let t19302 = t6514 * t937;
    let t19305 = t6455 * t937;
    (t19155, t19163, t19191, t19203, t19227, t19271, t19302, t19305)
}
