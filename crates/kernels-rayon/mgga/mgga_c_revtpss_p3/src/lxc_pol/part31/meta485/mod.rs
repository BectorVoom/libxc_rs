//! MGGA_C_REVTPSS lxc pol kernel — _part31_v4rho3sigma_6 meta485 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1774;
use chunk1::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1775;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_meta485(t25460: f64, t7150: f64, t11120: f64, t359: f64, t1982: f64, t994: f64, t1972: f64, t3223: f64, t1024: f64, t7125: f64, t3215: f64, t7117: f64, t3204: f64, t3143: f64, t3148: f64, sigma0: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t25461, t25464) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1774(t25460, t7150, t11120, t359);
        let (t25473, t25476, t25490, t25495, t25498, t25500, t25503, t25504) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1775(t1982, t25460, t994, t1972, t3223, t1024, t7125, t3215, t7117, t3204, t3143, t3148, sigma0);
    (t25461, t25464, t25473, t25476, t25490, t25495, t25498, t25500, t25503, t25504)
}
