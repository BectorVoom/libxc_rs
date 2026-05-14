//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 556/1141 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk556<F: Float>(t3886: F, t898: F, t353: F, t338: F, t1144: F, t1162: F, t3717: F, t376: F, t1118: F, t2204: F, t3769: F, t3785: F, t3790: F, t3795: F, t3797: F, t3801: F, t3807: F, t3813: F, t3822: F, t3834: F, t3843: F, t3860: F, t3869: F, t3882: F, t3883: F) -> (F, F, F, F, F, F, F) {
    let t3887 = t898 * t3886;
    let t3888 = t353 * t3887;
    let t3889 = t338 * t3888;
    let t3892 = t1144 * t1162;
    let t3893 = t338 * t3892;
    let t3896 = t376 * t3717;
    let t3897 = t353 * t3896;
    let t3898 = t338 * t3897;
    let t3902 = t1144 * t1118;
    let t3903 = t338 * t3902;
    let t3906 = t3807 + t3785 - t3795 + t3790 - t3769 - t3813 - t3797 + t3843 + t3883 + t3822 - t3882 - t3834 + t2204 + t3860 + t3801 - t3869;
    (t3887, t3889, t3893, t3896, t3898, t3903, t3906)
}
