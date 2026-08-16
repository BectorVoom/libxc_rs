//! MGGA_C_REVTPSS lxc pol kernel — _part30_v4rho3sigma_5 meta492 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1843;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_meta492(t26054: f64, t3917: f64, t25953: f64, t7284: f64, t1445: f64, t7242: f64, t689: f64, t7275: f64, t786: f64, t1364: f64, t26050: f64, t7289: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
        let (t26055, t26058, t26061, t26062, t26064, t26065, t26067) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1843(t26054, t3917, t25953, t7284, t1445, t7242, t689, t7275, t786, t1364, t26050, t7289);
    (t26055, t26058, t26061, t26062, t26064, t26065, t26067)
}
