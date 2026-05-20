//! MGGA_C_REVTPSS lxc pol kernel — _part32_v4rho3sigma_7 meta216 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk928;
use chunk1::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk929;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_meta216<F: Float>(t225: F, t5710: F, t1892: F, t213: F, t1357: F, t1904: F, t689: F, t1903: F, t72: F, t686: F, t3915: F, t1444: F, t4076: F, t1882: F, t555: F, t4086: F, t543: F, t2782: F, t1883: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t5711, t5715, t5718, t5719, t5721, t5722) = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk928::<F>(t225, t5710, t1892, t213, t1357, t1904, t689, t1903, t72, t686);
        let (t5723, t5728, t5735, t5737, t5738, t5740) = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk929::<F>(t3915, t5722, t1444, t1903, t4076, t1882, t555, t4086, t543, t2782, t1883, t72);
    (t5711, t5715, t5718, t5719, t5721, t5722, t5723, t5728, t5735, t5737, t5738, t5740)
}
