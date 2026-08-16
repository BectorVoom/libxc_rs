//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 18 (v4rho3sigma_6) CSE chunk 1176/1389 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part18_v4rho3sigma_6_chunk1176(t3717: f64, t3944: f64, t13919: f64, t3764: f64, t13917: f64, t1109: f64, t1193: f64, t353: f64, t859: f64, t1161: f64, t824: f64, t1113: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t15128 = t3944 * t3717;
    let t15134 = t13919 * t3764;
    let t15135 = t13917 * t15134;
    let t15137 = t1193 * t1109;
    let t15138 = t353 * t15137;
    let t15139 = t859 * t15138;
    let t15144 = t824 * t1161;
    let t15145 = t1113 * t15144;
    (t15128, t15134, t15135, t15137, t15138, t15139, t15144, t15145)
}
