//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 18 (v4rho3sigma_6) CSE chunk 883/1389 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part18_v4rho3sigma_6_chunk883<F: Float>(t1: F, t3360: F, t467: F, t6906: F, t3342: F, t4351: F, t418: F, t1351: F, t2477: F, t1523: F, t3346: F, t4358: F, t532: F) -> (F, F, F, F, F, F) {
    let t9762 = t3360 * t1;
    let t9763 = t9762 * t467;
    let t9764 = F::cast_from(0.18311555036753159941e-3_f64) * t9763;
    let t9765 = F::cast_from(0.13692109613355555556e1_f64) * t6906;
    let t9778 = t4351 * t3342;
    let t9779 = t9778 * t418;
    let t9781 = t2477 * t1351;
    let t9783 = t1523 * t3346;
    let t9784 = t9783 * t418;
    let t9788 = -F::cast_from(2.0_f64) * t532 - F::cast_from(6.0_f64) * t4358;
    (t9764, t9765, t9779, t9781, t9784, t9788)
}
