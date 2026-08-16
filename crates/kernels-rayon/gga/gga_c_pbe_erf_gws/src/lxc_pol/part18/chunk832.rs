//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 18 (v4rho3sigma_6) CSE chunk 832/1389 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part18_v4rho3sigma_6_chunk832(t2673: f64, t4934: f64, t639: f64, t219: f64, t5480: f64, t2679: f64, t2580: f64, t5125: f64, t1820: f64, t2756: f64, t579: f64, t532: f64) -> (f64, f64, f64, f64, f64) {
    let t7874 = t4934 * t2673;
    let t7876 = 32.0_f64 / 135.0_f64 * t639 * t7874;
    let t7877 = t5480 * t219;
    let t7878 = t7877 * t2679;
    let t7880 = 16.0_f64 / 81.0_f64 * t639 * t7878;
    let t7888 = t5125 * t2580;
    let t7890 = 32.0_f64 / 135.0_f64 * t1820 * t7888;
    let t7905 = 8.0_f64 / 45.0_f64 * t579 * t2756;
    let t7906 = 4.0_f64 * t532;
    (t7876, t7880, t7890, t7905, t7906)
}
