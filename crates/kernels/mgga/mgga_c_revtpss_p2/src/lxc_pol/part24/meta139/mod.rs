//! MGGA_C_REVTPSS lxc pol kernel — _part24_v4rho4_4 meta139 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk722;
use chunk1::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk723;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_meta139<F: Float>(t550: F, t72: F, t245: F, t125: F, t1882: F, t1873: F, t3957: F, t1892: F, t213: F, t1357: F, t1904: F, t689: F, t1903: F, t686: F, t3915: F, t555: F, t4086: F, t543: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t5673, t5674, t5681, t5715) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk722::<F>(t550, t72, t245, t125, t1882, t1873, t3957, t1892, t213);
        let (t5718, t5719, t5721, t5722, t5723, t5735, t5737) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk723::<F>(t1357, t1904, t689, t1903, t72, t686, t3915, t1882, t555, t4086, t543);
    (t5673, t5674, t5681, t5715, t5718, t5719, t5721, t5722, t5723, t5735, t5737)
}
