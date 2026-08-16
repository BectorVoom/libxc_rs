//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 19 (v4rho3sigma_7) CSE chunk 688/1404 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part19_v4rho3sigma_7_chunk688(t3879: f64, t867: f64, t866: f64, t3145: f64, t2266: f64, t2336: f64, t3271: f64, t3274: f64, t3302: f64, t3827: f64, t3834: f64, t3837: f64, t3843: f64, t3857: f64, t3860: f64, t3863: f64, t3869: f64, t3871: f64, t3875: f64, t902: f64, t914: f64, t929: f64) -> (f64, f64, f64, f64) {
    let t3880 = t867 * t3879;
    let t3882 = t866 * t3880 / 96.0_f64;
    let t3883 = 7.0_f64 / 144.0_f64 * t3145;
    let t3885 = -t914 * t3827 / 1536.0_f64 + 7.0_f64 / 576.0_f64 * t3271 - t3834 + t2266 * t3837 / 512.0_f64 + t3843 - t914 * t3857 / 1536.0_f64 + t3860 + t902 * t3863 / 1536.0_f64 - t3869 - t929 * t3871 / 768.0_f64 + 5.0_f64 / 768.0_f64 * t929 * t3875 - 7.0_f64 / 1152.0_f64 * t3274 - t3882 + t2336 + t3883 + 7.0_f64 / 1152.0_f64 * t3302;
    (t3880, t3882, t3883, t3885)
}
