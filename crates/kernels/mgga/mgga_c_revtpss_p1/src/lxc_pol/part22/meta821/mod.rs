//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta821 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2935;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2936;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta821<F: Float>(t22: F, t46389: F, t543: F, t5735: F, t1432: F, t5763: F, t9288: F, t1892: F, t3923: F, t2782: F, t4003: F, t5744: F, t10069: F, t14124: F, t14129: F, t14231: F, t10014: F, t14216: F, t13921: F, t4101: F, t686: F, t72: F, t10139: F, t136: F, t2457: F, t5659: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t47967, t47971, t47973, t47976) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2935::<F>(t22, t46389, t543, t5735, t1432, t5763, t9288, t1892, t3923, t2782, t4003, t5744);
        let (t47978, t47980, t47985, t47995, t47999, t48003) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2936::<F>(t10069, t14124, t14129, t14231, t10014, t14216, t13921, t4101, t686, t72, t10139, t136, t2457, t5659);
    (t47967, t47971, t47973, t47976, t47978, t47980, t47985, t47995, t47999, t48003)
}
