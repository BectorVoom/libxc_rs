//! MGGA_C_REVTPSS lxc pol kernel — _part31_v4rho3sigma_6 meta600 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2031;
use chunk1::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2032;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_meta600<F: Float>(t3917: F, t97783: F, t2435: F, t27986: F, t1904: F, t2439: F, t25916: F, t26050: F, t27884: F, t25304: F, t27883: F, t25946: F, t25898: F, t97699: F, t25901: F, t1364: F, t27961: F, t786: F, t2453: F, t3908: F, t7911: F, t136: F, t2457: F, t7920: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t97785, t97792, t97795, t97798, t97800) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2031::<F>(t3917, t97783, t2435, t27986, t1904, t2439, t25916, t26050, t27884, t25304, t27883, t25946);
        let (t97802, t97804, t97808, t97810, t97814) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2032::<F>(t25898, t97699, t25901, t1364, t27961, t786, t2453, t3908, t7911, t136, t2457, t7920);
    (t97785, t97792, t97795, t97798, t97800, t97802, t97804, t97808, t97810, t97814)
}
