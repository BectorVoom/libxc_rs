//! MGGA_C_REVTPSS lxc pol kernel — _part41_v4rho3tau_4 meta230 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk891;
use chunk1::mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk892;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_meta230<F: Float>(t300: F, t6212: F, t6185: F, t1642: F, t4719: F, t2986: F, t6189: F, t973: F, t981: F, t6205: F, t964: F, t3011: F, t3014: F, t3037: F, t4571: F, t6094: F, t6098: F, t6102: F, t341: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t6213, t6215, t6217, t6219, t6221, t6223, t6225, t6226) = mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk891::<F>(t300, t6212, t6185, t1642, t4719, t2986, t6189, t973, t981, t6205, t964, t3011);
        let (t6227, t6229, t6234, t6235) = mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk892::<F>(t3014, t6226, t981, t3037, t4571, t6094, t6098, t6102, t341);
    (t6213, t6215, t6217, t6219, t6221, t6223, t6225, t6226, t6227, t6229, t6234, t6235)
}
