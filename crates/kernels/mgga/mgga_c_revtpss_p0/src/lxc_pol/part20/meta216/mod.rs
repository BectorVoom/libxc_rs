//! MGGA_C_REVTPSS lxc pol kernel — _part20_v4rho4_0 meta216 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1001;
use chunk1::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1002;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_meta216<F: Float>(t231: F, t281: F, t68: F, t836: F, t10535: F, t2783: F, t860: F, t786: F, t2801: F, t2645: F, t268: F, t675: F, t2798: F, t10430: F, t10432: F, t10435: F, t10438: F, t10442: F, t10444: F, t10469: F, t9278: F, t9308: F, t9316: F, t9329: F) -> (F, F, F, F, F, F, F, F) {
        let (t10538, t10539, t10541, t10542, t10543, t10547) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1001::<F>(t231, t281, t68, t836, t10535, t2783, t860, t786, t2801, t2645, t268, t675);
        let (t10548, t10550) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1002::<F>(t10547, t2798, t10430, t10432, t10435, t10438, t10442, t10444, t10469, t9278, t9308, t9316, t9329);
    (t10538, t10539, t10541, t10542, t10543, t10547, t10548, t10550)
}
