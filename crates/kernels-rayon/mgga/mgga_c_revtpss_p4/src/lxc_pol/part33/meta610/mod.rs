//! MGGA_C_REVTPSS lxc pol kernel — _part33_v4rho3sigma_8 meta610 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2038;
use chunk1::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2039;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_meta610(t3917: f64, t97783: f64, t2435: f64, t27986: f64, t1904: f64, t2439: f64, t25916: f64, t26050: f64, t27884: f64, t25304: f64, t27883: f64, t25946: f64, t25898: f64, t97699: f64, t25901: f64, t1364: f64, t27961: f64, t786: f64, t2453: f64, t3908: f64, t7911: f64, t136: f64, t2457: f64, t7920: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t97785, t97792, t97795, t97798, t97800) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2038(t3917, t97783, t2435, t27986, t1904, t2439, t25916, t26050, t27884, t25304, t27883, t25946);
        let (t97802, t97804, t97808, t97810, t97814) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2039(t25898, t97699, t25901, t1364, t27961, t786, t2453, t3908, t7911, t136, t2457, t7920);
    (t97785, t97792, t97795, t97798, t97800, t97802, t97804, t97808, t97810, t97814)
}
