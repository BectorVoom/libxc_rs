//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta769 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2723;
use chunk1::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2724;
use chunk2::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2725;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta769<F: Float>(t2609: F, t4395: F, t14341: F, t2398: F, t40145: F, t11084: F, t15078: F, t40141: F, t4433: F, t4541: F, t50080: F, t50085: F, t50091: F, t50093: F, t50095: F, t50096: F, t45: F, t40148: F, t13312: F, t706: F, t750: F, t40150: F, t10326: F, t10356: F, t11231: F, t14447: F, t1490: F, t2251: F, t2258: F, t4227: F, t4230: F, t4328: F, t49889: F, t606: F, t766: F, t80: F, zeta_threshold: F, t57: F, t14458: F, t1491: F, t4232: F, t4235: F, t4335: F, t770: F, t83: F) -> (F, F, F, F, F, F, F, F, F) {
        let (t50098, t50100, t50101, t50102) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2723::<F>(t2609, t4395, t14341, t2398, t40145, t11084, t15078, t40141, t4433, t4541, t50080, t50085, t50091, t50093, t50095, t50096);
        let (t50106, t50114, t50115, t50132) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2724::<F>(t45, t40148, t13312, t706, t750, t40150, t10326, t10356, t11231, t14447, t1490, t2251, t2258, t4227, t4230, t4328, t49889, t606, t766, t80, zeta_threshold);
        let t50149 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2725::<F>(t57, t10326, t10356, t11231, t13312, t14458, t1491, t2251, t2258, t4232, t4235, t4335, t49889, t606, t770, t83, zeta_threshold);
    (t50098, t50100, t50101, t50102, t50106, t50114, t50115, t50132, t50149)
}
