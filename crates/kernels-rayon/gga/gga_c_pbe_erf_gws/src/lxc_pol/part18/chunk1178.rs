//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 18 (v4rho3sigma_6) CSE chunk 1178/1389 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part18_v4rho3sigma_6_chunk1178(t1123: f64, t15159: f64, t850: f64, t833: f64, t12109: f64, t2409: f64, t3965: f64, t1161: f64, t343: f64, t14724: f64, t13796: f64, t3989: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t15161 = t850 * t1123 * t15159;
    let t15162 = t15161 * t833;
    let t15164 = t2409 * t12109;
    let t15165 = t3965 * t15164;
    let t15167 = t343 * t1161;
    let t15168 = t14724 * t15167;
    let t15169 = t13796 * t15168;
    let t15170 = t3989 * t15169;
    (t15161, t15162, t15164, t15165, t15167, t15169, t15170)
}
