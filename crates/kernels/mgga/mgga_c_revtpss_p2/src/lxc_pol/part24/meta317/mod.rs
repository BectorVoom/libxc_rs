//! MGGA_C_REVTPSS lxc pol kernel — _part24_v4rho4_4 meta317 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1104;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_meta317<F: Float>(t1357: F, t6919: F, t689: F, t1904: F, t5599: F, t212: F, t6888: F, t1358: F, t6896: F, t6895: F, t72: F, t686: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t22409, t22410, t22427, t22428, t22445, t22446, t22447, t22449, t22450, t22452, t22453) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1104::<F>(t1357, t6919, t689, t1904, t5599, t212, t6888, t1358, t6896, t6895, t72, t686);
    (t22409, t22410, t22427, t22428, t22445, t22446, t22447, t22449, t22450, t22452, t22453)
}
