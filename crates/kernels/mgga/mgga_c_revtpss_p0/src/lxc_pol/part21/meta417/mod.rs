//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta417 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1897;
use chunk1::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1898;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta417<F: Float>(t1501: F, t2327: F, t648: F, t670: F, t2371: F, t93: F, t1514: F, t2289: F, t4264: F, t625: F, t4288: F, t10208: F, t1513: F, t2340: F, t2339: F, t4287: F) -> (F, F, F, F, F, F, F, F) {
        let (t13429, t13435) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1897::<F>(t1501, t2327, t648, t670);
        let (t13440, t13448, t13451, t13453, t13455, t13458) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1898::<F>(t2371, t93, t1514, t2289, t4264, t625, t4288, t10208, t1513, t2340, t2339, t4287);
    (t13429, t13435, t13440, t13448, t13451, t13453, t13455, t13458)
}
