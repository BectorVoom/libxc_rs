//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 27 (v4rho3sigma_3) CSE chunk 1723/2372 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1723<F: Float>(t22479: F, t510: F, t652: F, t1976: F, t2363: F, t2303: F, t71: F, t1863: F, t33: F, t9228: F, t43: F, t614: F) -> (F, F, F, F, F, F, F) {
    let t22480 = t510 * t22479;
    let t22482 = F::cast_from(2.0_f64) * t652 * t22480;
    let t22483 = t1976 * t2363;
    let t22489 = t71 * t2303;
    let t22490 = t1863 * t22489;
    let t22493 = t9228 * t33;
    let t22502 = t614 * t43;
    (t22480, t22482, t22483, t22489, t22490, t22493, t22502)
}
