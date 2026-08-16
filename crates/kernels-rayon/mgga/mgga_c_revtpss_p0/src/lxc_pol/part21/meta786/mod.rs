//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta786 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2833;
use chunk1::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2834;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta786(t11354: f64, t2881: f64, t4606: f64, t11358: f64, t15220: f64, t2897: f64, t918: f64, t2880: f64, t51849: f64, t51853: f64, t51858: f64, t51863: f64, t51867: f64, t51871: f64, t51875: f64, t15113: f64, t2889: f64, t11315: f64, t4598: f64, t15118: f64, t4614: f64, t11355: f64, t1600: f64, t41401: f64, t41382: f64, t13312: f64, t2852: f64, t606: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t51878, t51881, t51884, t51887, t51889) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2833(t11354, t2881, t4606, t11358, t15220, t2897, t918, t2880, t51849, t51853, t51858, t51863, t51867, t51871, t51875);
        let (t51890, t51892, t51894, t51896, t51899, t51902, t51905) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2834(t15113, t2889, t11315, t4598, t15118, t4614, t11355, t1600, t41401, t41382, t13312, t2852, t606);
    (t51878, t51881, t51884, t51887, t51889, t51890, t51892, t51894, t51896, t51899, t51902, t51905)
}
