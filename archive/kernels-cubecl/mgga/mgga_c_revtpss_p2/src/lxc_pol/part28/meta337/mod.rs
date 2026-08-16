//! MGGA_C_REVTPSS lxc pol kernel — _part28_v4rho3sigma_3 meta337 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1357;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_meta337<F: Float>(t2439: F, t931: F, t2915: F, t698: F, t2922: F, t913: F, t275: F, t290: F, t2925: F, t2935: F, t945: F, t2967: F, t941: F) -> (F, F, F, F, F, F) {
        let (t11366, t11368, t11385, t11387, t11399, t11404) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1357::<F>(t2439, t931, t2915, t698, t2922, t913, t275, t290, t2925, t2935, t945, t2967, t941);
    (t11366, t11368, t11385, t11387, t11399, t11404)
}
