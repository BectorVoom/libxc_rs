//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 20 (v4rho3sigma_8) CSE chunk 690/1389 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part20_v4rho3sigma_8_chunk690(t3886: f64, t898: f64, t353: f64, t338: f64, t1144: f64, t1162: f64, t3717: f64, t376: f64, t1118: f64, t2204: f64, t3769: f64, t3785: f64, t3790: f64, t3795: f64, t3797: f64, t3801: f64, t3807: f64, t3813: f64, t3822: f64, t3834: f64, t3843: f64, t3860: f64, t3869: f64, t3882: f64, t3883: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
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
    (t3887, t3888, t3889, t3892, t3893, t3896, t3897, t3898, t3902, t3903, t3906)
}
