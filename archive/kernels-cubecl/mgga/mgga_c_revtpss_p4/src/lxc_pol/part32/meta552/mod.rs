//! MGGA_C_REVTPSS lxc pol kernel — _part32_v4rho3sigma_7 meta552 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1869;
use chunk1::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1870;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_meta552<F: Float>(t94701: F, t96204: F, t26359: F, t9303: F, t13790: F, t4102: F, t685: F, t72: F, t1444: F, t5740: F, t675: F, t14109: F, t25900: F, t1892: F, t786: F, t25877: F, t14224: F, t689: F, t25304: F, t27883: F, t25898: F, t2453: F, t1955: F, t27836: F, t4075: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t96584, t96591, t97680, t97685, t97688) = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1869::<F>(t94701, t96204, t26359, t9303, t13790, t4102, t685, t72, t1444, t5740, t675, t14109, t25900);
        let (t97700, t97705, t97799, t97802, t97916, t97933) = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1870::<F>(t1892, t786, t25877, t14224, t689, t25304, t27883, t25898, t2453, t1955, t27836, t4075);
    (t96584, t96591, t97680, t97685, t97688, t97700, t97705, t97799, t97802, t97916, t97933)
}
