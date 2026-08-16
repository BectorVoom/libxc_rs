//! MGGA_C_REVTPSS lxc pol kernel — _part40_v4rho3tau_3 meta402 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk1478;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_meta402<F: Float>(t114: F, t31142: F, t8315: F, t2366: F, t8311: F, t104: F, t2357: F, t2358: F, t2362: F, t31035: F, t31134: F, t31135: F, t31137: F, t31139: F, t8258: F, t8267: F) -> (F, F, F, F, F, F) {
        let (t31143, t31146, t31149, t31150, t31153, t31157) = mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk1478::<F>(t114, t31142, t8315, t2366, t8311, t104, t2357, t2358, t2362, t31035, t31134, t31135, t31137, t31139, t8258, t8267);
    (t31143, t31146, t31149, t31150, t31153, t31157)
}
