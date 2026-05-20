//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta1077 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3858;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3859;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta1077<F: Float>(t47127: F, t47133: F, t47135: F, t48324: F, t48326: F, t47145: F, t47147: F, t47149: F, t48331: F, t48333: F, t48335: F, t40076: F, t40079: F, t47131: F, t47138: F, t47140: F, t47142: F, t47152: F, t13716: F, t13768: F, t13867: F, t13892: F, t13911: F, t13914: F, t13917: F, t1395: F, t1877: F, t1879: F, t22223: F, t22229: F, t22236: F, t225: F, t3889: F, t4049: F, t4050: F, t4053: F, t539: F, t541: F, t5644: F, t5650: F, t5651: F, t5655: F, t6832: F, t73345: F, t74099: F, t74100: F, t74102: F, t74103: F, t74113: F, t74127: F, t74140: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t74141, t74142, t74143, t74144, t74145, t74146, t74147, t74148, t74149, t74150, t74151, t74152) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3858::<F>(t47127, t47133, t47135, t48324, t48326, t47145, t47147, t47149, t48331, t48333, t48335, t40076, t40079, t47131, t47138, t47140, t47142, t47152);
        let t74165 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3859::<F>(t13716, t13768, t13867, t13892, t13911, t13914, t13917, t1395, t1877, t1879, t22223, t22229, t22236, t225, t3889, t4049, t4050, t4053, t539, t541, t5644, t5650, t5651, t5655, t6832, t73345, t74099, t74100, t74102, t74103, t74113, t74127, t74140, t74152);
    (t74141, t74142, t74143, t74144, t74145, t74146, t74147, t74148, t74149, t74150, t74151, t74165)
}
