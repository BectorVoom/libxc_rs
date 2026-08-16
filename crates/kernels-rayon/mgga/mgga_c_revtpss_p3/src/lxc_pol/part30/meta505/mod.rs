//! MGGA_C_REVTPSS lxc pol kernel — _part30_v4rho3sigma_5 meta505 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1884;
use chunk1::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1885;
use chunk2::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1886;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_meta505(t670: f64, t7683: f64, t2163: f64, t2371: f64, t2127: f64, t2165: f64, t2372: f64, t25193: f64, t25196: f64, t25804: f64, t25838: f64, t25840: f64, t25842: f64, t25844: f64, t25846: f64, t25853: f64, t25858: f64, t25860: f64, t25863: f64, t25868: f64, t26091: f64, t27060: f64, t3813: f64, t4151: f64, t651: f64, t671: f64, t7586: f64, t27075: f64, t3: f64, t1461: f64, t2170: f64, t26115: f64, t26117: f64, t26119: f64, t26122: f64, t26126: f64, t26129: f64, t26132: f64, t4162: f64, t4165: f64, t573: f64, t7696: f64, param_d: f64, t13426: f64, t1937: f64, t18227: f64, t4248: f64, t6993: f64, t7003: f64, t1518: f64, t648: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t27076, t27079, t27088) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1884(t670, t7683, t2163, t2371, t2127, t2165, t2372, t25193, t25196, t25804, t25838, t25840, t25842, t25844, t25846, t25853, t25858, t25860, t25863, t25868, t26091, t27060, t3813, t4151, t651, t671, t7586);
        let (t27089, t27090, t27102, t27110) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1885(t27075, t27088, t3, t1461, t2170, t26115, t26117, t26119, t26122, t26126, t26129, t26132, t4162, t4165, t573, t7696, param_d);
        let (t27116, t27118, t27120, t27122, t27123) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1886(t13426, t1937, t18227, t4248, t6993, t7003, t1518, t648);
    (t27076, t27079, t27089, t27090, t27102, t27110, t27116, t27118, t27120, t27122, t27123)
}
