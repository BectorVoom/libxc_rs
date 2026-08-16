//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 19 (v4rho3sigma_7) CSE chunk 1176/1404 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part19_v4rho3sigma_7_chunk1176(t15164: f64, t3965: f64, t1161: f64, t343: f64, t14724: f64, t13796: f64, t3989: f64, t1178: f64, t371: f64, t3887: f64, t1177: f64, t1118: f64, t1134: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t15165 = t3965 * t15164;
    let t15167 = t343 * t1161;
    let t15168 = t14724 * t15167;
    let t15169 = t13796 * t15168;
    let t15170 = t3989 * t15169;
    let t15177 = t371 * t1178 * t3887;
    let t15178 = t1177 * t15177;
    let t15181 = t1118 * t1134;
    (t15165, t15167, t15169, t15170, t15177, t15178, t15181)
}
