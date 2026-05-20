//! MGGA_C_REVTPSS lxc pol kernel — _part28_v4rho3sigma_3 meta352 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1372;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_meta352<F: Float>(t2434: F, t371: F, t373: F, t367: F, t3123: F, t3168: F, t3124: F, t3173: F, t1065: F, t675: F, t247: F, t906: F) -> (F, F, F, F, F, F) {
        let (t11970, t11972, t11977, t11980, t11986, t11988) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1372::<F>(t2434, t371, t373, t367, t3123, t3168, t3124, t3173, t1065, t675, t247, t906);
    (t11970, t11972, t11977, t11980, t11986, t11988)
}
