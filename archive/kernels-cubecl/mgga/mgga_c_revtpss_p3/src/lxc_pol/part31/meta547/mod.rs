//! MGGA_C_REVTPSS lxc pol kernel — _part31_v4rho3sigma_6 meta547 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1939;
use chunk1::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1940;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_meta547<F: Float>(t29731: F, t7160: F, t1668: F, t7817: F, t1089: F, t7821: F, t1646: F, t7810: F, t7145: F, t1976: F, t6350: F, t25464: F, t7828: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t29732, t29739, t29740, t29743, t29744, t29747, t29748, t29751) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1939::<F>(t29731, t7160, t1668, t7817, t1089, t7821, t1646, t7810, t7145, t1976, t6350);
        let (t29752, t29759) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1940::<F>(t25464, t29751, t1668, t7828);
    (t29732, t29739, t29740, t29743, t29744, t29747, t29748, t29751, t29752, t29759)
}
