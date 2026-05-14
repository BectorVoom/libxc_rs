//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 889/1141 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk889<F: Float>(t10024: F, t2096: F, t11387: F, t331: F, t4395: F, t3916: F, t6154: F, t1114: F, t3747: F, t4422: F, t833: F, t3703: F, t898: F, t3717: F, t20091: F, t3744: F) -> (F, F, F, F, F, F, F) {
    let t35137 = t10024 * t2096;
    let t35187 = t11387 * t331;
    let t35188 = t4395 * t35187;
    let t35277 = t3916 * t6154;
    let t35481 = t1114 * t3747 * t4422 * t833;
    let t35541 = t898 * t3703;
    let t35553 = t898 * t3717;
    let t35638 = t20091 * t3744;
    (t35137, t35188, t35277, t35481, t35541, t35553, t35638)
}
