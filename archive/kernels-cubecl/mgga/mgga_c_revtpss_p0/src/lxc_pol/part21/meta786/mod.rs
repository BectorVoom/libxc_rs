//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta786 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2833;
use chunk1::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2834;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta786<F: Float>(t11354: F, t2881: F, t4606: F, t11358: F, t15220: F, t2897: F, t918: F, t2880: F, t51849: F, t51853: F, t51858: F, t51863: F, t51867: F, t51871: F, t51875: F, t15113: F, t2889: F, t11315: F, t4598: F, t15118: F, t4614: F, t11355: F, t1600: F, t41401: F, t41382: F, t13312: F, t2852: F, t606: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t51878, t51881, t51884, t51887, t51889) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2833::<F>(t11354, t2881, t4606, t11358, t15220, t2897, t918, t2880, t51849, t51853, t51858, t51863, t51867, t51871, t51875);
        let (t51890, t51892, t51894, t51896, t51899, t51902, t51905) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2834::<F>(t15113, t2889, t11315, t4598, t15118, t4614, t11355, t1600, t41401, t41382, t13312, t2852, t606);
    (t51878, t51881, t51884, t51887, t51889, t51890, t51892, t51894, t51896, t51899, t51902, t51905)
}
