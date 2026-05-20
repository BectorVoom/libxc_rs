//! MGGA_C_REVTPSS lxc pol kernel — _part40_v4rho3tau_3 meta311 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk1083;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_meta311<F: Float>(t3010: F, t320: F, t315: F, t11132: F, t11337: F, t963: F, t3013: F, t323: F, t3006: F, t3014: F, t2873: F, t910: F) -> (F, F, F, F, F, F, F, F, F) {
        let (t11465, t11466, t11479, t11480, t11506, t11507, t11509, t11524, t11528) = mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk1083::<F>(t3010, t320, t315, t11132, t11337, t963, t3013, t323, t3006, t3014, t2873, t910);
    (t11465, t11466, t11479, t11480, t11506, t11507, t11509, t11524, t11528)
}
