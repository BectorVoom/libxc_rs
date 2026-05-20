//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta516 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2149;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta516<F: Float>(t3168: F, t4878: F, t13392: F, t4801: F, t1042: F, t11150: F, t3181: F, t15936: F, t4806: F, t11144: F, t11852: F, t3124: F, t4820: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t16190, t16195, t16196, t16199, t16200, t16201, t16204, t16205, t16208, t16209, t16210, t16218) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2149::<F>(t3168, t4878, t13392, t4801, t1042, t11150, t3181, t15936, t4806, t11144, t11852, t3124, t4820);
    (t16190, t16195, t16196, t16199, t16200, t16201, t16204, t16205, t16208, t16209, t16210, t16218)
}
