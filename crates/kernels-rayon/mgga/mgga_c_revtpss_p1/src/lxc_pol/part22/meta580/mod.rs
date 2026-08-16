//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta580 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2437;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2438;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta580(t18281: f64, t190: f64, t706: f64, t14441: f64, t10593: f64, t10597: f64, t189: f64, t5819: f64, t606: f64, t14330: f64, t10608: f64, t4308: f64, t4311: f64, t10613: f64, t10592: f64, t10596: f64, t10604: f64, t10611: f64, t14433: f64, t14618: f64, t9524: f64, t9542: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t18569, t18571, t18572, t18573, t18574, t18575, t18576, t18578, t18579, t18581) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2437(t18281, t190, t706, t14441, t10593, t10597, t189, t5819, t606, t14330, t10608, t4308, t4311);
        let (t18582, t18583) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2438(t10613, t10592, t10596, t10604, t10611, t14433, t14618, t18571, t18572, t18573, t18574, t18578, t18579, t18581, t9524, t9542);
    (t18569, t18571, t18572, t18573, t18574, t18575, t18576, t18578, t18579, t18581, t18582, t18583)
}
