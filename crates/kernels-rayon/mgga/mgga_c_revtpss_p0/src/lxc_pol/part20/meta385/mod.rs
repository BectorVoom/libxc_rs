//! MGGA_C_REVTPSS lxc pol kernel — _part20_v4rho4_0 meta385 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1414;
use chunk1::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1415;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_meta385(t39521: f64, t41141: f64, t41150: f64, t41168: f64, t41174: f64, t41185: f64, t41191: f64, t41208: f64, t2985: f64, t3010: f64, t2988: f64, t11509: f64, t981: f64, t3013: f64, t11616: f64, t3022: f64, t241: f64, t281: f64, t283: f64, t11144: f64, t2251: f64, t2258: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t41211, t41224, t41225) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1414(t39521, t41141, t41150, t41168, t41174, t41185, t41191, t41208, t2985, t3010, t2988);
        let (t41229, t41235, t41238, t41241, t41243, t41245, t41246, t41248) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1415(t11509, t41224, t41225, t981, t3010, t3013, t11616, t3022, t241, t281, t283, t11144, t2251, t2258);
    (t41211, t41224, t41225, t41229, t41235, t41238, t41241, t41243, t41245, t41246, t41248)
}
