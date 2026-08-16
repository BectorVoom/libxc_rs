//! MGGA_C_REVTPSS lxc pol kernel — _part30_v4rho3sigma_5 meta525 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1938;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_meta525(t25082: f64, t28067: f64, t4237: f64, t76: f64, t13269: f64, t38: f64, t1497: f64, t640: f64, t77: f64, t4241: f64, t84: f64, t1470: f64, t2242: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t28069, t28089, t28093, t28104, t28105, t28108, t28109, t28112) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1938(t25082, t28067, t4237, t76, t13269, t38, t1497, t640, t77, t4241, t84, t1470, t2242);
    (t28069, t28089, t28093, t28104, t28105, t28108, t28109, t28112)
}
