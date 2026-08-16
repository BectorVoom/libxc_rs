//! MGGA_C_REVTPSS lxc pol kernel — _part30_v4rho3sigma_5 meta587 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2044;
use chunk1::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2045;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_meta587(t25875: f64, t94762: f64, t4004: f64, t676: f64, t25880: f64, t25894: f64, t25877: f64, t94382: f64, t94590: f64, t25950: f64, t26050: f64, t25304: f64, t25949: f64, t25946: f64, t25878: f64, t94661: f64, t7246: f64, t9692: f64, t26054: f64, t9671: f64, t1419: f64, t7063: f64, t25898: f64, t25901: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t94766, t94769, t94771, t94772, t94774, t94776) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2044(t25875, t94762, t4004, t676, t25880, t25894, t25877, t94382, t94590, t25950, t26050, t25304, t25949);
        let (t94777, t94779, t94784, t94799, t94801, t94802, t94803) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2045(t25946, t94776, t25878, t94661, t7246, t9692, t26054, t9671, t1419, t7063, t25898, t25901);
    (t94766, t94769, t94771, t94772, t94774, t94777, t94779, t94784, t94799, t94801, t94802, t94803)
}
