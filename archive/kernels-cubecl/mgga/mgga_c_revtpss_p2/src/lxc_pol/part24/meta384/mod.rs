//! MGGA_C_REVTPSS lxc pol kernel — _part24_v4rho4_4 meta384 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1286;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_meta384<F: Float>(t1211: F, t24713: F, t1828: F, t6587: F, t1277: F, t6573: F, t24543: F, t487: F, t13143: F, t24864: F, t489: F, t1287: F, t1794: F, t6695: F) -> (F, F, F, F, F, F, F, F) {
        let (t24892, t24899, t24900, t24906, t24911, t24912, t24915, t24919) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1286::<F>(t1211, t24713, t1828, t6587, t1277, t6573, t24543, t487, t13143, t24864, t489, t1287, t1794, t6695);
    (t24892, t24899, t24900, t24906, t24911, t24912, t24915, t24919)
}
