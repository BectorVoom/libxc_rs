//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 765/1340 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk765<F: Float>(t326: F, t6523: F, t6458: F, t2370: F, t5728: F, t941: F, t410: F, t6514: F, t6012: F, t6517: F, t2363: F, t937: F) -> (F, F, F, F, F, F, F, F) {
    let t6524 = t6523 * t326;
    let t6525 = t6524 * t6458;
    let t6526 = t5728 * t2370;
    let t6545 = t941 * t941;
    let t6546 = F::cast_from(1.0_f64) / t6545;
    let t6555 = t6514 * t410;
    let t6557 = t6012 * t6517;
    let t6561 = t2363 * t937;
    (t6524, t6525, t6526, t6545, t6546, t6555, t6557, t6561)
}
