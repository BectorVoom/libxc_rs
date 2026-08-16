//! MGGA_C_REVTPSS lxc pol kernel — _part28_v4rho3sigma_3 meta518 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1937;
use chunk1::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1938;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_meta518<F: Float>(t1096: F, t7817: F, t7160: F, t7821: F, t988: F, t7145: F, t1035: F, t7810: F, t1043: F, t1089: F, t1982: F, t27418: F, t342: F, t1678: F, t3140: F, t1078: F, t1668: F, t25681: F, t4866: F, t7168: F, t7828: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t27594, t27595, t27598, t27599, t27604, t27606, t27609) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1937::<F>(t1096, t7817, t7160, t7821, t988, t7145, t1035, t7810, t1043, t1089, t1982, t27418);
        let (t27616, t27621, t27627, t27631, t27634) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1938::<F>(t342, t7810, t1678, t3140, t1078, t1982, t1089, t1668, t25681, t4866, t7168, t7828, t988);
    (t27594, t27595, t27598, t27599, t27604, t27606, t27609, t27616, t27621, t27627, t27631, t27634)
}
