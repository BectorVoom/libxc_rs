//! MGGA_C_REVTPSS lxc pol kernel — _part29_v4rho3sigma_4 meta581 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1932;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_meta581<F: Float>(t25207: F, t61102: F, t14365: F, t14436: F, t18875: F, t94245: F, t25759: F, t61203: F, t98674: F, t98759: F, t98651: F, t15071: F, t33: F) -> (F, F, F, F, F, F, F, F) {
        let (t99558, t100858, t100944, t100947, t100953, t100958, t100964, t100969) = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1932::<F>(t25207, t61102, t14365, t14436, t18875, t94245, t25759, t61203, t98674, t98759, t98651, t15071, t33);
    (t99558, t100858, t100944, t100947, t100953, t100958, t100964, t100969)
}
