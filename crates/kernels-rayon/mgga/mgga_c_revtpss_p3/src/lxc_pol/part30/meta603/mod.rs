//! MGGA_C_REVTPSS lxc pol kernel — _part30_v4rho3sigma_5 meta603 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2064;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_meta603(t12904: f64, t7618: f64, t3666: f64, t7623: f64, t12808: f64, t29096: f64, t3655: f64, t7610: f64, t1256: f64, t26817: f64, t12898: f64, t2139: f64) -> (f64, f64, f64, f64, f64, f64) {
        let (t97247, t97250, t97261, t97267, t97269, t97272) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2064(t12904, t7618, t3666, t7623, t12808, t29096, t3655, t7610, t1256, t26817, t12898, t2139);
    (t97247, t97250, t97261, t97267, t97269, t97272)
}
