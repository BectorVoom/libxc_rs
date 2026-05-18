//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 1161/1278 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk1161<F: Float>(t13364: F, t31195: F, t38850: F, t1988: F, t9687: F, t2001: F, t6361: F, t5561: F, t5946: F, t1755: F, t30644: F, t5792: F, t7822: F) -> (F, F, F, F, F, F, F) {
    let t40095 = t31195 * t13364 * t38850;
    let t40099 = t1988 * t9687;
    let t40101 = t2001 * t6361;
    let t40105 = t2001 * t5561;
    let t40107 = t2001 * t5946;
    let t40109 = t30644 * t1755;
    let t40111 = t7822 * t5792;
    (t40095, t40099, t40101, t40105, t40107, t40109, t40111)
}
