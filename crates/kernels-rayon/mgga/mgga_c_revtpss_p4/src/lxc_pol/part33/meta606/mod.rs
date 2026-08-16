//! MGGA_C_REVTPSS lxc pol kernel — _part33_v4rho3sigma_8 meta606 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2030;
use chunk1::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2031;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_meta606(t26866: f64, t3746: f64, t12904: f64, t7618: f64, t3666: f64, t7623: f64, t12808: f64, t29096: f64, t3655: f64, t7610: f64, t12898: f64, t2139: f64, t12984: f64, t7613: f64, t12966: f64, t2138: f64, t12851: f64, t2134: f64, t3567: f64, t8945: f64, t26894: f64, t29199: f64, t3596: f64, t37885: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t97232, t97247, t97250, t97261, t97267, t97272) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2030(t26866, t3746, t12904, t7618, t3666, t7623, t12808, t29096, t3655, t7610, t12898, t2139);
        let (t97288, t97292, t97296, t97304, t97308, t97312) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2031(t12984, t7613, t12966, t2138, t12851, t2134, t3567, t8945, t26894, t29199, t3596, t37885);
    (t97232, t97247, t97250, t97261, t97267, t97272, t97288, t97292, t97296, t97304, t97308, t97312)
}
