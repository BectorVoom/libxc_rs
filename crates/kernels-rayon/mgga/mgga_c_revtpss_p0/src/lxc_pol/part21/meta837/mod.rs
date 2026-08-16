//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta837 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3137;
use chunk1::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3138;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta837(t12248: f64, t1732: f64, t12415: f64, t12222: f64, t5192: f64, t1196: f64, t45289: f64, t5205: f64, t12235: f64, t16673: f64, t3531: f64, t12361: f64, t16655: f64, t16658: f64, t44101: f64, t12243: f64, t16665: f64, t16669: f64, t44012: f64, t3384: f64, t3427: f64, t5105: f64, t12571: f64, t5198: f64, t12485: f64, t3524: f64, t5180: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t57820, t57822, t57825, t57827, t57829, t57831) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3137(t12248, t1732, t12415, t12222, t5192, t1196, t45289, t5205, t12235, t16673, t3531, t12361, t16655);
        let (t57833, t57835, t57837, t57840, t57842, t57846) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3138(t16658, t44101, t12243, t16665, t16669, t44012, t3384, t3427, t5105, t12571, t5198, t1196, t12485, t3524, t5180);
    (t57820, t57822, t57825, t57827, t57829, t57831, t57833, t57835, t57837, t57840, t57842, t57846)
}
