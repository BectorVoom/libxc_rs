//! MGGA_C_PKZB lxc pol — lxc_pol part 10 (v4rho4_2) CSE chunk 1211/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part10_v4rho4_2_chunk1211<F: Float>(t3022: F, t500: F, t218: F, t675: F, t7992: F, t7984: F, t7988: F, t1180: F, t5555: F, t1878: F, t3061: F, t3065: F, t8235: F, t832: F, t1184: F, t6142: F) -> (F, F, F, F, F, F, F, F, F) {
    let t22258 = 8.0 * t3022 * t500;
    let t22265 = t218 * t675 * t7992;
    let t22284 = t218 * t675 * t7984;
    let t22287 = t218 * t675 * t7988;
    let t22290 = t218 * t5555 * t1180;
    let t22293 = t218 * t1878 * t3061;
    let t22296 = t218 * t1878 * t3065;
    let t22357 = t8235 * t832;
    let t22391 = t6142 * t1184;
    (t22258, t22265, t22284, t22287, t22290, t22293, t22296, t22357, t22391)
}
