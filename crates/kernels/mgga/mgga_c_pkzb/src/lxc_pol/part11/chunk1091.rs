//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 1091/1340 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk1091<F: Float>(t466: F, t931: F, t53: F, t6404: F, t414: F, t6545: F, t6523: F, t937: F, t6514: F, t6455: F, t2463: F, t23: F, t4810: F) -> (F, F, F, F, F, F, F, F) {
    let t19191 = t466 * t931;
    let t19203 = t53 * t6404;
    let t19227 = F::cast_from(1.0_f64) / t6545 / t414;
    let t19271 = t6523 * t937;
    let t19302 = t6514 * t937;
    let t19305 = t6455 * t937;
    let t19338 = t2463 * t2463;
    let t19339 = F::cast_from(1.0_f64) / t19338;
    let t19377 = t23 * t4810;
    (t19191, t19203, t19227, t19271, t19302, t19305, t19339, t19377)
}
