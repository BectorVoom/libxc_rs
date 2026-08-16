//! MGGA_C_REVTPSS lxc pol kernel — _part32_v4rho3sigma_7 meta326 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1245;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_meta326(t3475: f64, t431: f64, t426: f64, t12295: f64, t12351: f64, t1159: f64, t3478: f64, t434: f64, t1175: f64, t3520: f64, t3519: f64, t444: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
        let (t12429, t12459, t12460, t12470, t12472, t12481, t12485) = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1245(t3475, t431, t426, t12295, t12351, t1159, t3478, t434, t1175, t3520, t3519, t444);
    (t12429, t12459, t12460, t12470, t12472, t12481, t12485)
}
