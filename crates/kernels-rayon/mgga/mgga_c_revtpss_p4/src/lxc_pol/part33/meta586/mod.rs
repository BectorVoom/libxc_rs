//! MGGA_C_REVTPSS lxc pol kernel — _part33_v4rho3sigma_8 meta586 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1999;
use chunk1::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2000;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_meta586(t10982: f64, t1949: f64, t9646: f64, t2471: f64, t25355: f64, t10985: f64, t25422: f64, t25335: f64, t9303: f64, t1959: f64, t41117: f64, t68: f64, t785: f64, t251: f64, t281: f64, t25410: f64, t7078: f64, t2453: f64, t2458: f64, t7049: f64, t1950: f64, t2769: f64, t786: f64, t25404: f64, t40270: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t93206, t93207, t93210, t93224, t93231, t93238) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1999(t10982, t1949, t9646, t2471, t25355, t10985, t25422, t25335, t9303, t1959, t41117, t68, t785);
        let (t93240, t93242, t93252, t93261, t93272) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2000(t251, t281, t93238, t25410, t7078, t2453, t2458, t7049, t1950, t2769, t786, t25404, t40270);
    (t93206, t93207, t93210, t93224, t93231, t93238, t93240, t93242, t93252, t93261, t93272)
}
