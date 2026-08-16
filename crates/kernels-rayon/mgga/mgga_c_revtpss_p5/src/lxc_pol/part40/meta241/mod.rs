//! MGGA_C_REVTPSS lxc pol kernel — _part40_v4rho3tau_3 meta241 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk920;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_meta241(t225: f64, t494: f64, t5412: f64, t1811: f64, t460: f64, t1214: f64, t1828: f64, t1277: f64, t1294: f64, t3737: f64, t1284: f64, t1770: f64) -> (f64, f64, f64, f64, f64) {
        let (t5414, t5417, t5423, t5429, t5436) = mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk920(t225, t494, t5412, t1811, t460, t1214, t1828, t1277, t1294, t3737, t1284, t1770);
    (t5414, t5417, t5423, t5429, t5436)
}
