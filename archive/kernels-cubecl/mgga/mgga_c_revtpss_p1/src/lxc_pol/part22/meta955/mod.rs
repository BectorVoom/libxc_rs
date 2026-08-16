//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta955 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3199;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta955<F: Float>(t10355: F, t44: F, t10368: F, t56: F, t1518: F, t670: F, t1913: F, t4168: F, t18217: F, t571: F, t1921: F, t4153: F) -> (F, F, F, F, F, F) {
        let (t60308, t60311, t60595, t60607, t60609, t60611) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3199::<F>(t10355, t44, t10368, t56, t1518, t670, t1913, t4168, t18217, t571, t1921, t4153);
    (t60308, t60311, t60595, t60607, t60609, t60611)
}
