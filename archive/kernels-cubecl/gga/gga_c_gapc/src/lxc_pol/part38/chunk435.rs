//! GGA_C_GAPC lxc pol — lxc_pol part 38 (v4rho2sigma2_17) CSE chunk 435/1307 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part38_v4rho2sigma2_17_chunk435<F: Float>(t2361: F, t852: F, t4: F, t748: F, t78: F, t1365: F, t854: F, t106: F, t737: F, t2059: F, t2078: F, t858: F) -> (F, F, F, F, F) {
    let t2362 = t852 * t2361;
    let t2364 = t4 * t78 * t748;
    let t2367 = t854 * t1365;
    let t2370 = t106 * t737;
    let t2371 = t2370 * t2059;
    let t2374 = t858 * t2078;
    (t2362, t2364, t2367, t2371, t2374)
}
