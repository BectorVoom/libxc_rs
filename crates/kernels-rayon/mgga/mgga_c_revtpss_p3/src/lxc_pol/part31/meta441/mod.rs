//! MGGA_C_REVTPSS lxc pol kernel — _part31_v4rho3sigma_6 meta441 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1572;
use chunk1::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1573;
use chunk2::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1574;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_meta441(t19971: f64, t4893: f64, t3117: f64, t11922: f64, t6272: f64, t3115: f64, t1668: f64, t3181: f64, t372: f64, t1045: f64, t4574: f64, t12131: f64, t6266: f64, t15691: f64, t1011: f64, t1068: f64, t15689: f64, t15700: f64, t19951: f64, t19954: f64, t19957: f64, t19960: f64, t19963: f64, t19968: f64, t3106: f64, t4892: f64, t6331: f64, t4579: f64, t1043: f64, t1592: f64, t3155: f64, t4817: f64, t4834: f64, t11933: f64, t11956: f64, t11967: f64, t11972: f64, t11989: f64, t15830: f64, t16121: f64, t16226: f64, t1675: f64, t3211: f64, t6273: f64, t6278: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t19973, t19976, t19977, t19982, t19985) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1572(t19971, t4893, t3117, t11922, t6272, t3115, t1668, t3181, t372, t1045, t4574, t12131, t6266);
        let (t19986, t19989) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1573(t15691, t19985, t1011, t1068, t15689, t15700, t19951, t19954, t19957, t19960, t19963, t19968, t19973, t19977, t19982, t3106, t4892, t6331);
        let (t19993, t19998, t20012) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1574(t1045, t4579, t15691, t1043, t1592, t3155, t4817, t4834, t11933, t11956, t11967, t11972, t11989, t15700, t15830, t16121, t16226, t1675, t3211, t6273, t6278);
    (t19973, t19976, t19982, t19986, t19989, t19993, t19998, t20012)
}
