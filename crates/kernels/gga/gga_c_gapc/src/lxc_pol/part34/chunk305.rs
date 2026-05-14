//! GGA_C_GAPC lxc pol — lxc_pol part 34 (v4rho2sigma2_13) CSE chunk 305/1210 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part34_v4rho2sigma2_13_chunk305<F: Float>(t27: F, t345: F, t13: F, t362: F, t363: F) -> (F, F) {
    let t1185 = t345 * t27;
    let t1186 = 1.0 / t1185;
    let t1187 = t13 * t1186;
    let t1188 = t362 * t362;
    let t1189 = t1188 * t363;
    let t1191 = 2.0 * t1187 * t1189;
    (t1188, t1191)
}
