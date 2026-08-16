//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta417 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1897;
use chunk1::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1898;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta417(t1501: f64, t2327: f64, t648: f64, t670: f64, t2371: f64, t93: f64, t1514: f64, t2289: f64, t4264: f64, t625: f64, t4288: f64, t10208: f64, t1513: f64, t2340: f64, t2339: f64, t4287: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t13429, t13435) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1897(t1501, t2327, t648, t670);
        let (t13440, t13448, t13451, t13453, t13455, t13458) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1898(t2371, t93, t1514, t2289, t4264, t625, t4288, t10208, t1513, t2340, t2339, t4287);
    (t13429, t13435, t13440, t13448, t13451, t13453, t13455, t13458)
}
