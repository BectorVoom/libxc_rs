//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta451 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2113;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2114;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta451<F: Float>(t15154: F, t2908: F, t141: F, t15158: F, t930: F, t4625: F, t698: F, t4622: F, t15130: F, t15137: F, t15142: F, t15147: F, t15151: F, t15156: F, t15160: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t15162, t15163, t15165, t15166, t15168, t15169, t15170) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2113::<F>(t15154, t2908, t141, t15158, t930, t4625, t698, t4622);
        let (t15172, t15173, t15175) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2114::<F>(t15130, t2908, t141, t15137, t15142, t15147, t15151, t15156, t15160, t15163, t15166, t15169, t15170);
    (t15162, t15163, t15165, t15166, t15168, t15169, t15170, t15172, t15173, t15175)
}
