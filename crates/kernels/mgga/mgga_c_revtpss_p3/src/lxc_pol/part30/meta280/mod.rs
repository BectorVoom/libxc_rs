//! MGGA_C_REVTPSS lxc pol kernel — _part30_v4rho3sigma_5 meta280 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1232;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_meta280<F: Float>(t532: F, t7933: F, t1450: F, t2014: F, t2034: F, t5542: F, t1916: F, t2042: F, t1518: F, t7330: F, t572: F, t117: F, t7741: F) -> (F, F, F, F, F, F, F, F, F) {
        let (t7934, t7935, t7936, t7937, t7938, t7949, t7950, t7952, t7953) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1232::<F>(t532, t7933, t1450, t2014, t2034, t5542, t1916, t2042, t1518, t7330, t572, t117, t7741);
    (t7934, t7935, t7936, t7937, t7938, t7949, t7950, t7952, t7953)
}
