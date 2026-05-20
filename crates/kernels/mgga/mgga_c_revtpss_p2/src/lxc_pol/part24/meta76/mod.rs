//! MGGA_C_REVTPSS lxc pol kernel — _part24_v4rho4_4 meta76 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk465;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_meta76<F: Float>(t2289: F, t45: F, t631: F, t78: F, t57: F, t635: F, t81: F, t112: F, t111: F, t654: F, t99: F, t107: F) -> (F, F, F, F, F, F, F, F, F) {
        let (t2290, t2297, t2299, t2304, t2306, t2335, t2339, t2349, t2357) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk465::<F>(t2289, t45, t631, t78, t57, t635, t81, t112, t111, t654, t99, t107);
    (t2290, t2297, t2299, t2304, t2306, t2335, t2339, t2349, t2357)
}
