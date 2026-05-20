//! MGGA_C_REVTPSS lxc pol kernel — _part24_v4rho4_4 meta430 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1380;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_meta430<F: Float>(t45936: F, t584: F, t596: F, t20: F, t2237: F, t12: F, t14: F, t27: F, t10285: F, t2231: F, t10293: F, t592: F) -> (F, F, F, F, F, F, F) {
        let (t45937, t45939, t45941, t45944, t45946, t45948, t45949) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1380::<F>(t45936, t584, t596, t20, t2237, t12, t14, t27, t10285, t2231, t10293, t592);
    (t45937, t45939, t45941, t45944, t45946, t45948, t45949)
}
