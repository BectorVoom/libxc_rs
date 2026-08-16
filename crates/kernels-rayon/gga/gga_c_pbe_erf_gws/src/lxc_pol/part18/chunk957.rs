//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 18 (v4rho3sigma_6) CSE chunk 957/1389 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part18_v4rho3sigma_6_chunk957(t3342: f64, t4951: f64, t418: f64, t5264: f64, t2560: f64, t34: f64, t1856: f64, t3421: f64, t606: f64, t2554: f64, t4949: f64, t11: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t10783 = t4951 * t3342;
    let t10784 = t10783 * t418;
    let t10785 = t5264 * t10784;
    let t10788 = t2560 * t34;
    let t10789 = t1856 * t10788;
    let t10792 = t3421 * t418;
    let t10793 = t606 * t10792;
    let t10796 = t2554 * t34;
    let t10797 = t606 * t10796;
    let t10800 = t4949 * t10784;
    let t10801 = t11 * t10800;
    (t10784, t10785, t10788, t10789, t10792, t10793, t10796, t10797, t10801)
}
