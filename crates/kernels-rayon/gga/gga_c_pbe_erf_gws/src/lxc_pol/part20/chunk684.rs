//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 20 (v4rho3sigma_8) CSE chunk 684/1389 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part20_v4rho3sigma_8_chunk684(t343: f64, t3824: f64, t904: f64, t916: f64, t858: f64, t867: f64, t866: f64, t2157: f64, t2155: f64, t339: f64, t3703: f64, t3717: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t3825 = t3824 * t343;
    let t3826 = t904 * t3825;
    let t3827 = t916 * t3826;
    let t3831 = t858 * t3825;
    let t3832 = t867 * t3831;
    let t3834 = t866 * t3832 / 96.0_f64;
    let t3835 = t3824 * t2157;
    let t3836 = t904 * t3835;
    let t3837 = t916 * t3836;
    let t3840 = t858 * t3835;
    let t3841 = t867 * t3840;
    let t3843 = t2155 * t3841 / 48.0_f64;
    let t3848 = t339 * t3703;
    let t3851 = t339 * t3717;
    (t3825, t3827, t3831, t3832, t3834, t3835, t3837, t3840, t3841, t3843, t3848, t3851)
}
