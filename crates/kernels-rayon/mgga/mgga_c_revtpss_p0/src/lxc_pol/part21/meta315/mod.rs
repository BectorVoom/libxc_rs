//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta315 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1588;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta315(t136: f64, t860: f64, t2457: f64, t2710: f64, t10519: f64, t10524: f64, t10533: f64, t10539: f64, t10543: f64, t10548: f64, t10639: f64, t10645: f64, t10647: f64, t10651: f64, t10655: f64, t10657: f64, t10661: f64, t10666: f64, t10910: f64, t213: f64, t234: f64, t2646: f64, t2724: f64, t2815: f64, t820: f64, t837: f64, t879: f64) -> (f64, f64, f64) {
        let (t10914, t10916, t10918) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1588(t136, t860, t2457, t2710, t10519, t10524, t10533, t10539, t10543, t10548, t10639, t10645, t10647, t10651, t10655, t10657, t10661, t10666, t10910, t213, t234, t2646, t2724, t2815, t820, t837, t879);
    (t10914, t10916, t10918)
}
