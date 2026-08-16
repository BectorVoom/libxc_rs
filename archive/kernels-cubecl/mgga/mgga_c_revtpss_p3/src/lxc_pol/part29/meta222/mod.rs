//! MGGA_C_REVTPSS lxc pol kernel — _part29_v4rho3sigma_4 meta222 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk970;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_meta222<F: Float>(t1043: F, t3154: F, t4893: F, t3117: F, t3317: F, t4891: F, t357: F, t1651: F, t1045: F, t999: F, t4781: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t4894, t4895, t4896, t4899, t4900, t4901, t4902, t4905, t4906, t4907, t4910, t4911) = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk970::<F>(t1043, t3154, t4893, t3117, t3317, t4891, t357, t1651, t1045, t999, t4781);
    (t4894, t4895, t4896, t4899, t4900, t4901, t4902, t4905, t4906, t4907, t4910, t4911)
}
