//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 16 (v4rho3sigma_4) CSE chunk 828/1361 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part16_v4rho3sigma_4_chunk828<F: Float>(t810: F, t898: F, t938: F, t353: F, t4386: F, t2239: F, t2246: F, t329: F, t369: F, t838: F, t2404: F, t2052: F, t381: F) -> (F, F, F, F) {
    let t6794 = t898 * t810;
    let t6795 = t6794 * t938;
    let t6796 = t353 * t6795;
    let t6797 = t4386 * t6796;
    let t6805 = t2246 * t2239;
    let t6832 = t329 * t838 * t369;
    let t6833 = t6832 * t2404;
    let t6854 = F::cast_from(1.0_f64) / t2052 / t381;
    (t6797, t6805, t6833, t6854)
}
