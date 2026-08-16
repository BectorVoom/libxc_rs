//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta341 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1668;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta341<F: Float>(t11467: F, t11509: F, t2962: F, t955: F, t2970: F, t953: F, t11114: F, t11118: F, t11399: F, t11404: F, t11409: F, t11411: F, t11445: F, t11450: F, t11453: F, t11456: F, t11461: F, t11466: F, t11468: F, t11502: F, t11507: F, t2938: F, t2943: F, t2963: F, t2968: F, t2971: F, t2982: F, t3007: F, t3015: F, t946: F, t965: F, t974: F) -> (F, F, F, F) {
        let (t11510, t11513, t11517, t11520) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1668::<F>(t11467, t11509, t2962, t955, t2970, t953, t11114, t11118, t11399, t11404, t11409, t11411, t11445, t11450, t11453, t11456, t11461, t11466, t11468, t11502, t11507, t2938, t2943, t2963, t2968, t2971, t2982, t3007, t3015, t946, t965, t974);
    (t11510, t11513, t11517, t11520)
}
