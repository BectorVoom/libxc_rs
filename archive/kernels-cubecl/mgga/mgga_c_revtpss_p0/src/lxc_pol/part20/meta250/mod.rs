//! MGGA_C_REVTPSS lxc pol kernel — _part20_v4rho4_0 meta250 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1083;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_meta250<F: Float>(t1046: F, t11262: F, t1041: F, t1038: F, t3229: F, t1036: F, t1033: F, t3169: F, t3173: F, t3140: F, t989: F, t3149: F) -> (F, F, F, F, F, F, F) {
        let (t11263, t11264, t11267, t11268, t11271, t11273, t11274) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1083::<F>(t1046, t11262, t1041, t1038, t3229, t1036, t1033, t3169, t3173, t3140, t989, t3149);
    (t11263, t11264, t11267, t11268, t11271, t11273, t11274)
}
