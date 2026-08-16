//! MGGA_C_REVTPSS lxc pol kernel — _part20_v4rho4_0 meta392 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1445;
use chunk1::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1446;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_meta392<F: Float>(t41372: F, t916: F, t270: F, t276: F, t39484: F, t41383: F, t2880: F, t41386: F, t11318: F, t698: F, t141: F, t41314: F, t930: F, t11354: F, t2881: F, t2889: F, t11315: F, t11372: F, t11358: F, t11375: F, t41316: F, t41323: F, t41353: F, t41356: F, t41359: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t41396, t41402, t41404, t41406, t41409) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1445::<F>(t41372, t916, t270, t276, t39484, t41383, t2880, t41386, t11318, t698, t141, t41314, t930);
        let (t41412, t41414, t41417, t41419, t41421) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1446::<F>(t11354, t2881, t2889, t11315, t11372, t11358, t11375, t41316, t41323, t41353, t41356, t41359, t41396, t41402, t41404, t41406, t41409);
    (t41396, t41402, t41404, t41406, t41409, t41412, t41414, t41417, t41419, t41421)
}
