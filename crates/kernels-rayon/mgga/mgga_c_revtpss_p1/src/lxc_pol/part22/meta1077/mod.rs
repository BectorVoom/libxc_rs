//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta1077 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3858;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3859;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta1077(t47127: f64, t47133: f64, t47135: f64, t48324: f64, t48326: f64, t47145: f64, t47147: f64, t47149: f64, t48331: f64, t48333: f64, t48335: f64, t40076: f64, t40079: f64, t47131: f64, t47138: f64, t47140: f64, t47142: f64, t47152: f64, t13716: f64, t13768: f64, t13867: f64, t13892: f64, t13911: f64, t13914: f64, t13917: f64, t1395: f64, t1877: f64, t1879: f64, t22223: f64, t22229: f64, t22236: f64, t225: f64, t3889: f64, t4049: f64, t4050: f64, t4053: f64, t539: f64, t541: f64, t5644: f64, t5650: f64, t5651: f64, t5655: f64, t6832: f64, t73345: f64, t74099: f64, t74100: f64, t74102: f64, t74103: f64, t74113: f64, t74127: f64, t74140: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t74141, t74142, t74143, t74144, t74145, t74146, t74147, t74148, t74149, t74150, t74151, t74152) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3858(t47127, t47133, t47135, t48324, t48326, t47145, t47147, t47149, t48331, t48333, t48335, t40076, t40079, t47131, t47138, t47140, t47142, t47152);
        let t74165 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3859(t13716, t13768, t13867, t13892, t13911, t13914, t13917, t1395, t1877, t1879, t22223, t22229, t22236, t225, t3889, t4049, t4050, t4053, t539, t541, t5644, t5650, t5651, t5655, t6832, t73345, t74099, t74100, t74102, t74103, t74113, t74127, t74140, t74152);
    (t74141, t74142, t74143, t74144, t74145, t74146, t74147, t74148, t74149, t74150, t74151, t74165)
}
