//! MGGA_C_REVTPSS lxc pol kernel — _part24_v4rho4_4 meta237 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk998;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_meta237<F: Float>(t1412: F, t1882: F, t2470: F, t5721: F, t3915: F, t2435: F, t5600: F, t1426: F, t1893: F, t786: F, t136: F, t1903: F) -> (F, F, F, F, F, F, F) {
        let (t14045, t14090, t14091, t14097, t14099, t14100, t14103) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk998::<F>(t1412, t1882, t2470, t5721, t3915, t2435, t5600, t1426, t1893, t786, t136, t1903);
    (t14045, t14090, t14091, t14097, t14099, t14100, t14103)
}
