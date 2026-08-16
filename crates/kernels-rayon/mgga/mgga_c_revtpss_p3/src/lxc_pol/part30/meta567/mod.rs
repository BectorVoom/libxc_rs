//! MGGA_C_REVTPSS lxc pol kernel — _part30_v4rho3sigma_5 meta567 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2014;
use chunk1::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2015;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_meta567(t2471: f64, t25355: f64, t10985: f64, t25422: f64, t25335: f64, t9303: f64, t25425: f64, t689: f64, t25431: f64, t25411: f64, t1959: f64, t41117: f64, t68: f64, t785: f64, t251: f64, t281: f64, t25410: f64, t7078: f64, t2453: f64, t2458: f64, t7049: f64, t1950: f64, t2769: f64, t786: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t93207, t93210, t93224, t93226, t93228, t93231) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2014(t2471, t25355, t10985, t25422, t25335, t9303, t25425, t689, t25431, t25411, t1959, t41117);
        let (t93238, t93240, t93242, t93252, t93261) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2015(t68, t785, t251, t281, t25410, t7078, t2453, t2458, t7049, t1950, t2769, t786);
    (t93207, t93210, t93224, t93226, t93228, t93231, t93238, t93240, t93242, t93252, t93261)
}
