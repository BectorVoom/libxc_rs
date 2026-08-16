//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta811 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2656;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta811<F: Float>(t19127: F, t2926: F, t2873: F, t6104: F, t11108: F, t6396: F, t19226: F, t2970: F, t2986: F, t6184: F, t11509: F, t6205: F) -> (F, F, F, F, F, F) {
        let (t63650, t63677, t63907, t63971, t63997, t64043) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2656::<F>(t19127, t2926, t2873, t6104, t11108, t6396, t19226, t2970, t2986, t6184, t11509, t6205);
    (t63650, t63677, t63907, t63971, t63997, t64043)
}
