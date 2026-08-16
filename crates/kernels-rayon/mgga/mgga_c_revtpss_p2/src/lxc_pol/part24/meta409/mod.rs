//! MGGA_C_REVTPSS lxc pol kernel — _part24_v4rho4_4 meta409 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1351;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_meta409(t10115: f64, t251: f64, t2410: f64, t2985: f64, t3010: f64, t3013: f64, t241: f64, t281: f64, t283: f64, t2297: f64, t2851: f64, t11821: f64, t240: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t41117, t41154, t41224, t41235, t41238, t41245, t41246, t41270, t41294) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1351(t10115, t251, t2410, t2985, t3010, t3013, t241, t281, t283, t2297, t2851, t11821, t240);
    (t41117, t41154, t41224, t41235, t41238, t41245, t41246, t41270, t41294)
}
