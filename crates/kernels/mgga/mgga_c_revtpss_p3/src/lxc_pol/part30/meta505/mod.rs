//! MGGA_C_REVTPSS lxc pol kernel — _part30_v4rho3sigma_5 meta505 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1884;
use chunk1::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1885;
use chunk2::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1886;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_meta505<F: Float>(t670: F, t7683: F, t2163: F, t2371: F, t2127: F, t2165: F, t2372: F, t25193: F, t25196: F, t25804: F, t25838: F, t25840: F, t25842: F, t25844: F, t25846: F, t25853: F, t25858: F, t25860: F, t25863: F, t25868: F, t26091: F, t27060: F, t3813: F, t4151: F, t651: F, t671: F, t7586: F, t27075: F, t3: F, t1461: F, t2170: F, t26115: F, t26117: F, t26119: F, t26122: F, t26126: F, t26129: F, t26132: F, t4162: F, t4165: F, t573: F, t7696: F, param_d: F, t13426: F, t1937: F, t18227: F, t4248: F, t6993: F, t7003: F, t1518: F, t648: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t27076, t27079, t27088) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1884::<F>(t670, t7683, t2163, t2371, t2127, t2165, t2372, t25193, t25196, t25804, t25838, t25840, t25842, t25844, t25846, t25853, t25858, t25860, t25863, t25868, t26091, t27060, t3813, t4151, t651, t671, t7586);
        let (t27089, t27090, t27102, t27110) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1885::<F>(t27075, t27088, t3, t1461, t2170, t26115, t26117, t26119, t26122, t26126, t26129, t26132, t4162, t4165, t573, t7696, param_d);
        let (t27116, t27118, t27120, t27122, t27123) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1886::<F>(t13426, t1937, t18227, t4248, t6993, t7003, t1518, t648);
    (t27076, t27079, t27089, t27090, t27102, t27110, t27116, t27118, t27120, t27122, t27123)
}
