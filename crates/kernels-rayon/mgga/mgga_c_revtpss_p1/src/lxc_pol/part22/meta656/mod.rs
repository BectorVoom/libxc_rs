//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta656 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2608;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2609;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta656(t1248: f64, t6573: f64, t1250: f64, t3720: f64, t19666: f64, t5302: f64, t1042: f64, t17550: f64, t19661: f64, t1715: f64, t17500: f64, t5056: f64, t5277: f64, t20261: f64, t20263: f64, t20386: f64, t20388: f64, t20390: f64, t20393: f64, t20396: f64, t20399: f64, t20402: f64, t20404: f64, t20450: f64, t20452: f64, t20454: f64, t20471: f64, t20475: f64, t20477: f64, t20685: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t20856, t20857, t20858, t20863, t20864, t20867, t20868, t20875, t20876, t20879) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2608(t1248, t6573, t1250, t3720, t19666, t5302, t1042, t17550, t19661, t1715, t17500, t5056, t5277);
        let (t20880, t20885) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2609(t1042, t20879, t20261, t20263, t20386, t20388, t20390, t20393, t20396, t20399, t20402, t20404, t20450, t20452, t20454, t20471, t20475, t20477, t20685);
    (t20856, t20857, t20858, t20863, t20864, t20867, t20868, t20875, t20876, t20879, t20880, t20885)
}
