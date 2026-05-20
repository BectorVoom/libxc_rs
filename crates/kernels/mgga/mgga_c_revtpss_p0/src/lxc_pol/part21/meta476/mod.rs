//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta476 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2042;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta476<F: Float>(t1614: F, t2942: F, t11354: F, t1600: F, t2881: F, t11358: F, t2880: F, t4606: F, t918: F, t2889: F, t4598: F, t2897: F) -> (F, F, F, F, F, F, F, F, F) {
        let (t15104, t15107, t15108, t15110, t15111, t15113, t15114, t15116, t15118) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2042::<F>(t1614, t2942, t11354, t1600, t2881, t11358, t2880, t4606, t918, t2889, t4598, t2897);
    (t15104, t15107, t15108, t15110, t15111, t15113, t15114, t15116, t15118)
}
