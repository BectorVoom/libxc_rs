//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 19 (v4rho3sigma_7) CSE chunk 1128/1222 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part19_v4rho3sigma_7_chunk1128<F: Float>(t12237: F, t13780: F, t14637: F, t3990: F, t13796: F, t3737: F, t875: F, t3722: F, t3989: F, t52926: F, t9872: F, t13917: F, t3258: F, t3757: F, t51021: F, t938: F) -> (F, F, F, F, F) {
    let t56374 = t14637 * t3990 * t13780 * t12237;
    let t56404 = t14637 * t13796 * t3737 * t875;
    let t56431 = t3989 * t13796 * t3722 * t875;
    let t56434 = t14637 * t52926 * t9872;
    let t56439 = t13917 * t51021 * t3258 * t3757 * t938;
    (t56374, t56404, t56431, t56434, t56439)
}
