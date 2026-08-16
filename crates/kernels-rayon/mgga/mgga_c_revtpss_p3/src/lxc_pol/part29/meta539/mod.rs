//! MGGA_C_REVTPSS lxc pol kernel — _part29_v4rho3sigma_4 meta539 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1871;
use chunk1::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1872;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_meta539(t2453: f64, t26496: f64, t10506: f64, t10510: f64, t26497: f64, t10073: f64, t25402: f64, t7056: f64, t7398: f64, t26481: f64, t93182: f64, t25411: f64, t2754: f64, t676: f64, t136: f64, t2457: f64, t7423: f64, t25299: f64, t25431: f64, t26555: f64, t40270: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t95773, t95774, t95779, t95783, t95785, t95786) = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1871(t2453, t26496, t10506, t10510, t26497, t10073, t25402, t7056, t7398, t26481, t93182, t25411);
        let (t95790, t95793, t95794, t95796, t95798, t95807) = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1872(t26481, t2754, t676, t25411, t136, t2457, t7423, t25299, t25431, t95785, t26555, t40270);
    (t95773, t95774, t95779, t95783, t95786, t95790, t95793, t95794, t95796, t95798, t95807)
}
