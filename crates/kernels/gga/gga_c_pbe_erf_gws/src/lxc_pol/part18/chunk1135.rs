//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 18 (v4rho3sigma_6) CSE chunk 1135/1210 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part18_v4rho3sigma_6_chunk1135<F: Float>(t13796: F, t3722: F, t3989: F, t875: F, t14637: F, t52926: F, t9872: F, t13917: F, t3258: F, t3757: F, t51021: F, t938: F, t11572: F, t50998: F, t51066: F, t1144: F, t14402: F, t4386: F) -> (F, F, F, F, F) {
    let t56431 = t3989 * t13796 * t3722 * t875;
    let t56434 = t14637 * t52926 * t9872;
    let t56439 = t13917 * t51021 * t3258 * t3757 * t938;
    let t56442 = t50998 * t51066 * t11572;
    let t56445 = t4386 * t1144 * t14402;
    (t56431, t56434, t56439, t56442, t56445)
}
