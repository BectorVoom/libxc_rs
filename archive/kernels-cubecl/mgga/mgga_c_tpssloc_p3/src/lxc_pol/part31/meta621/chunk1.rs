//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 31 (v4rho3sigma_7) CSE chunk 1875/2041 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1875<F: Float>(t1985: F, t1998: F, t20009: F, t214: F, t1352: F, t26331: F, t6976: F, t97011: F, t1799: F, t6637: F, t6888: F, t90809: F) -> (F, F, F) {
    let t97055 = t1985 * t214 * t1998 * t20009;
    let t97059 = t26331 * t6976 * t97011 * t1352;
    let t97063 = t6888 * t6637 * t90809 * t1799;
    (t97055, t97059, t97063)
}
