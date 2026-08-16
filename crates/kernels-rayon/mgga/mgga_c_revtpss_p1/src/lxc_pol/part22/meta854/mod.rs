//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta854 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2997;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2998;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta854(t10115: f64, t1570: f64, t11007: f64, t1579: f64, t252: f64, t2771: f64, t2782: f64, t4322: f64, t9292: f64, t2772: f64, t4321: f64, t689: f64, t11024: f64, t1580: f64, t10981: f64, t22: f64, t868: f64, t15060: f64, t2435: f64, t14982: f64, t2465: f64, t2470: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t50155, t50161, t50164, t50166, t50169) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2997(t10115, t1570, t11007, t1579, t252, t2771, t2782, t4322, t9292, t2772, t4321, t689);
        let (t50174, t50178, t50183, t50186) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2998(t11024, t1580, t689, t10981, t1579, t22, t868, t15060, t2435, t14982, t2465, t2470);
    (t50155, t50161, t50164, t50166, t50169, t50174, t50178, t50183, t50186)
}
