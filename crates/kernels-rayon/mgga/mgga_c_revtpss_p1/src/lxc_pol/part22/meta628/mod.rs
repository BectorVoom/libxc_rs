//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta628 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2544;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta628(t1045: f64, t4579: f64, t15691: f64, t1043: f64, t1592: f64, t3155: f64, t4817: f64, t4834: f64, t11933: f64, t11956: f64, t11967: f64, t11972: f64, t11989: f64, t15700: f64, t15830: f64, t16121: f64, t16226: f64, t1675: f64, t3211: f64, t6273: f64, t6278: f64) -> (f64, f64, f64, f64, f64) {
        let (t19992, t19993, t19997, t19998, t20012) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2544(t1045, t4579, t15691, t1043, t1592, t3155, t4817, t4834, t11933, t11956, t11967, t11972, t11989, t15700, t15830, t16121, t16226, t1675, t3211, t6273, t6278);
    (t19992, t19993, t19997, t19998, t20012)
}
