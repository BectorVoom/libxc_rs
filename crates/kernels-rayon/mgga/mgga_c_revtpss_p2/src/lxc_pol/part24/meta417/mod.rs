//! MGGA_C_REVTPSS lxc pol kernel — _part24_v4rho4_4 meta417 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1363;
use chunk1::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1364;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_meta417(t1086: f64, t11200: f64, t3090: f64, t16565: f64, t994: f64, t42859: f64, t42862: f64, t342: f64, t3145: f64, t368: f64, t42871: f64, t42872: f64, t1035: f64, t357: f64, t3057: f64, t4980: f64, t3286: f64, t4995: f64, t3143: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t43291, t43341, t43347, t43351) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1363(t1086, t11200, t3090, t16565, t994, t42859, t42862, t342, t3145, t368, t42871);
        let (t43352, t43401, t43402, t43438, t43446, t43456, t43471) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1364(t42872, t43351, t1035, t42859, t342, t357, t3057, t4980, t11200, t3286, t4995, t3143);
    (t43291, t43341, t43347, t43351, t43352, t43401, t43402, t43438, t43446, t43456, t43471)
}
