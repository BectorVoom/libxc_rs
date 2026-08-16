//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta237 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1396;
use chunk1::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1397;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta237(t3: f64, t5789: f64, t116: f64, t1518: f64, t670: f64, t117: f64, t4292: f64, t1459: f64, t1461: f64, t1916: f64, t1918: f64, t572: f64, t573: f64, param_d: f64, t159: f64, t793: f64, t1448: f64, t4147: f64, t1493: f64, t76: f64, t587: f64, t65: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t5790, t5795, t5801, t5802, t5805, t5808) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1396(t3, t5789, t116, t1518, t670, t117, t4292, t1459, t1461, t1916, t1918, t572, t573, param_d);
        let (t7021, t7315, t7719, t8779) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1397(t159, t793, t1448, t4147, t1493, t76, t587, t65);
    (t5790, t5795, t5801, t5802, t5805, t5808, t7021, t7315, t7719, t8779)
}
