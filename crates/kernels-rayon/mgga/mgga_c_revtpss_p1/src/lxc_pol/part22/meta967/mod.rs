//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta967 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3231;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3232;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta967(t14622: f64, t18259: f64, t18281: f64, t189: f64, t4401: f64, t606: f64, t190: f64, t2611: f64, t60717: f64, t18555: f64, t2619: f64, t13396: f64, t14330: f64, t4402: f64, t50113: f64, t40150: f64, t14341: f64, t4311: f64, t18253: f64, t18268: f64, t198: f64, t2394: f64, t2430: f64, t262: f64, t39989: f64, t4541: f64, t50080: f64, t5966: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t61265, t61269, t61274, t61283, t61286) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3231(t14622, t18259, t18281, t189, t4401, t606, t190, t2611, t60717, t18555, t2619, t13396, t14330, t4402);
        let (t61287, t61288, t61290, t61291) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3232(t50113, t40150, t14341, t4311, t18253, t18268, t198, t2394, t2430, t262, t39989, t4541, t50080, t5966, t61265, t61269, t61274, t61283, t61286);
    (t61265, t61269, t61274, t61283, t61286, t61287, t61288, t61290, t61291)
}
