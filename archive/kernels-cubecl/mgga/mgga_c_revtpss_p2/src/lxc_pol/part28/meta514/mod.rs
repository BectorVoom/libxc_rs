//! MGGA_C_REVTPSS lxc pol kernel — _part28_v4rho3sigma_3 meta514 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1925;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_meta514<F: Float>(t27426: F, t7160: F, t1043: F, t1089: F, t7817: F, t7821: F, t1096: F, t7810: F, t988: F, t7145: F, t4820: F, t7122: F) -> (F, F, F, F, F, F, F, F) {
        let (t27427, t27433, t27437, t27440, t27441, t27444, t27445, t27448) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1925::<F>(t27426, t7160, t1043, t1089, t7817, t7821, t1096, t7810, t988, t7145, t4820, t7122);
    (t27427, t27433, t27437, t27440, t27441, t27444, t27445, t27448)
}
