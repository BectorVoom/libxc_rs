//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta777 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2581;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta777(t58145: f64, t58225: f64, t3432: f64, t5060: f64, t12226: f64, t1719: f64, t56228: f64, t56176: f64, t56183: f64, t12555: f64, t5180: f64, t12486: f64, t300: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t58411, t58452, t58466, t58473, t58536, t58543, t58607, t58609, t58624, t58647, t58665) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2581(t58145, t58225, t3432, t5060, t12226, t1719, t56228, t56176, t56183, t12555, t5180, t12486, t300);
    (t58411, t58452, t58466, t58473, t58536, t58543, t58607, t58609, t58624, t58647, t58665)
}
