//! MGGA_C_REVTPSS lxc pol kernel — _part24_v4rho4_4 meta498 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1499;
use chunk1::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1500;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_meta498<F: Float>(t4321: F, t6072: F, t689: F, t23383: F, t2465: F, t686: F, t72: F, t10995: F, t23403: F, t212: F, t23359: F, t780: F, t23177: F, t2798: F, t14568: F, t18730: F, t14586: F, t6016: F, t10529: F, t2782: F, t233: F, t869: F) -> (F, F, F, F, F, F, F, F) {
        let (t76051, t76058, t76062, t76081) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1499::<F>(t4321, t6072, t689, t23383, t2465, t686, t72, t10995, t23403, t212, t23359, t780);
        let (t76100, t76104, t76108, t76117) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1500::<F>(t23177, t2798, t686, t72, t14568, t18730, t14586, t6016, t10529, t2782, t233, t23359, t689, t869);
    (t76051, t76058, t76062, t76081, t76100, t76104, t76108, t76117)
}
