//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta825 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2943;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2944;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta825<F: Float>(t5552: F, t588: F, t5560: F, t13581: F, t177: F, t762: F, t1317: F, t13632: F, t3857: F, t5569: F, t512: F, t749: F, t5567: F, t13672: F, t2608: F, t5566: F, t1856: F, t9544: F, t13597: F, t2516: F, t2626: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t48185, t48212, t48222, t48225, t48227, t48230) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2943::<F>(t5552, t588, t5560, t13581, t177, t762, t1317, t13632, t3857, t5569, t512, t749);
        let (t48235, t48237, t48240, t48243, t48255, t48260) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2944::<F>(t3857, t5567, t1317, t13672, t2608, t512, t5566, t1856, t9544, t13597, t2516, t2626);
    (t48185, t48212, t48222, t48225, t48227, t48230, t48235, t48237, t48240, t48243, t48255, t48260)
}
