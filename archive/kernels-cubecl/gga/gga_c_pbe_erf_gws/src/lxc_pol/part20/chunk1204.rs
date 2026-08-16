//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 20 (v4rho3sigma_8) CSE chunk 1204/1389 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part20_v4rho3sigma_8_chunk1204<F: Float>(t19898: F, t3912: F, t4384: F, t1161: F, t26654: F, t1114: F, t3747: F, t4383: F, t12227: F, t331: F, t11387: F, t11660: F, t6472: F) -> (F, F, F, F, F, F) {
    let t35000 = t3912 * t19898;
    let t35003 = t3912 * t4384;
    let t35023 = t26654 * t1161;
    let t35057 = t1114 * t3747 * t4383;
    let t35171 = t12227 * t331;
    let t35187 = t11387 * t331;
    let t35193 = t11660 * t6472 * t35171;
    (t35000, t35003, t35023, t35057, t35187, t35193)
}
