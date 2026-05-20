//! MGGA_C_REVTPSS lxc pol kernel — _part31_v4rho3sigma_6 meta441 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1572;
use chunk1::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1573;
use chunk2::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1574;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_meta441<F: Float>(t19971: F, t4893: F, t3117: F, t11922: F, t6272: F, t3115: F, t1668: F, t3181: F, t372: F, t1045: F, t4574: F, t12131: F, t6266: F, t15691: F, t1011: F, t1068: F, t15689: F, t15700: F, t19951: F, t19954: F, t19957: F, t19960: F, t19963: F, t19968: F, t3106: F, t4892: F, t6331: F, t4579: F, t1043: F, t1592: F, t3155: F, t4817: F, t4834: F, t11933: F, t11956: F, t11967: F, t11972: F, t11989: F, t15830: F, t16121: F, t16226: F, t1675: F, t3211: F, t6273: F, t6278: F) -> (F, F, F, F, F, F, F, F) {
        let (t19973, t19976, t19977, t19982, t19985) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1572::<F>(t19971, t4893, t3117, t11922, t6272, t3115, t1668, t3181, t372, t1045, t4574, t12131, t6266);
        let (t19986, t19989) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1573::<F>(t15691, t19985, t1011, t1068, t15689, t15700, t19951, t19954, t19957, t19960, t19963, t19968, t19973, t19977, t19982, t3106, t4892, t6331);
        let (t19993, t19998, t20012) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1574::<F>(t1045, t4579, t15691, t1043, t1592, t3155, t4817, t4834, t11933, t11956, t11967, t11972, t11989, t15700, t15830, t16121, t16226, t1675, t3211, t6273, t6278);
    (t19973, t19976, t19982, t19986, t19989, t19993, t19998, t20012)
}
