//! MGGA_C_REVTPSS lxc pol kernel — _part31_v4rho3sigma_6 meta433 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1550;
use chunk1::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1551;
use chunk2::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1552;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_meta433(t19501: f64, t3095: f64, t3092: f64, t1043: f64, t3155: f64, t6271: f64, t3117: f64, t12131: f64, t357: f64, t4786: f64, t6100: f64, t1065: f64, t6244: f64, t906: f64, t1042: f64, t3172: f64, t6301: f64, t1041: f64, t5819: f64, t606: f64, t16199: f64, t1469: f64, t4186: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t19626, t19636, t19641, t19645, t19649) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1550(t19501, t3095, t3092, t1043, t3155, t6271, t3117, t12131, t357, t4786, t6100, t1065, t6244);
        let (t19651, t19658, t19659, t19661) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1551(t19649, t906, t1042, t3172, t6301, t1041, t5819, t606);
        let (t19663, t19666) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1552(t16199, t19661, t1042, t1469, t4186);
    (t19626, t19636, t19641, t19645, t19651, t19658, t19659, t19661, t19663, t19666)
}
