//! MGGA_C_REVTPSS lxc pol kernel — _part32_v4rho3sigma_7 meta556 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1875;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_meta556<F: Float>(t13985: F, t94423: F, t13878: F, t25972: F, t94479: F, t2689: F, t27936: F, t13857: F, t94564: F, t25978: F, t5629: F, t1885: F, t94459: F) -> (F, F, F, F, F, F, F) {
        let (t98202, t98206, t98217, t98218, t98220, t98222, t98224) = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1875::<F>(t13985, t94423, t13878, t25972, t94479, t2689, t27936, t13857, t94564, t25978, t5629, t1885, t94459);
    (t98202, t98206, t98217, t98218, t98220, t98222, t98224)
}
