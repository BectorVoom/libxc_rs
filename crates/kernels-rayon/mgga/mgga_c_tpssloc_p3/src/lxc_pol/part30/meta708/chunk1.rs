//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 30 (v4rho3sigma_6) CSE chunk 2337/2341 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2337(t12524: f64, t28896: f64, t3941: f64, t5493: f64, t6534: f64, t100902: f64, t100908: f64, t100911: f64, t100915: f64, t100917: f64, t100921: f64, t100924: f64, t100927: f64, t100929: f64, t100932: f64, t100934: f64, t100936: f64, t1458: f64, t19534: f64, t20181: f64, t23880: f64, t5376: f64, t671: f64, t7010: f64, t86647: f64, t86656: f64) -> f64 {
    let t100938 = 54.0_f64 * t12524 * t28896;
    let t100941 = 27.0_f64 * t3941 * t6534 * t5493;
    let t100942 = t100902 + 54.0_f64 * t86647 * t5376 + 0.135e2_f64 * t7010 * t19534 + t100908 + 27.0_f64 * t23880 * t20181 + 0.135e2_f64 * t100911 * t671 + t100915 + t100917 + 27.0_f64 * t86656 * t1458 + t100921 + t100924 + t100927 + t100929 + t100932 + t100934 + t100936 + t100938 + t100941;
    t100942
}
