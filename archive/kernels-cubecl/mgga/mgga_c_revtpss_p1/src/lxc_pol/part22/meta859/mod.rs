//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta859 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3007;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3008;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta859<F: Float>(t10696: F, t1544: F, t14832: F, t2394: F, t2661: F, t14668: F, t14923: F, t124: F, t4423: F, t14686: F, t14931: F, t4366: F, t2645: F, t2722: F, t1558: F, t231: F, t40406: F, t685: F, t72: F, t826: F, t14869: F, t9775: F, t10899: F, t136: F, t216: F) -> (F, F, F, F, F, F, F, F, F) {
        let (t50399, t50409, t50412, t50415) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3007::<F>(t10696, t1544, t14832, t2394, t2661, t14668, t14923, t124, t4423, t14686, t14931, t4366);
        let (t50418, t50423, t50436, t50443, t50446) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3008::<F>(t1544, t2645, t2722, t1558, t231, t40406, t685, t72, t826, t14869, t9775, t10899, t136, t216);
    (t50399, t50409, t50412, t50415, t50418, t50423, t50436, t50443, t50446)
}
