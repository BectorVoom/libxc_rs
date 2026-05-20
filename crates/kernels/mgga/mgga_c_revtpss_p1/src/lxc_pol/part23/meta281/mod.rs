//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta281 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1506;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta281<F: Float>(t10631: F, t808: F, t10886: F, t2699: F, t798: F, t802: F, t159: F, t853: F, t216: F, t2729: F, t794: F) -> (F, F, F, F, F, F, F) {
        let (t10887, t10888, t10890, t10891, t10899, t10900, t10905) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1506::<F>(t10631, t808, t10886, t2699, t798, t802, t159, t853, t216, t2729, t794);
    (t10887, t10888, t10890, t10891, t10899, t10900, t10905)
}
