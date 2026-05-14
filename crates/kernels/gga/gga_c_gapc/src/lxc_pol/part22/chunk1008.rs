//! GGA_C_GAPC lxc pol — lxc_pol part 22 (v4rho2sigma2_1) CSE chunk 1008/1209 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part22_v4rho2sigma2_1_chunk1008<F: Float>(t10047: F, t11387: F, t3402: F, t11808: F, t9865: F, t11875: F, t1453: F, t7949: F, t818: F, t959: F, t1736: F, t640: F, t7073: F, t1086: F, t2211: F, t9388: F) -> (F, F, F, F, F, F) {
    let t33588 = t3402 * t11387 * t10047;
    let t33590 = t11808 * t9865;
    let t33595 = t11875 * t1453 * t818 * t959 * t7949;
    let t33597 = t640 * t1736;
    let t33598 = t7073 * t33597;
    let t33601 = t33598 * t1086 * t2211 * t9388;
    (t33588, t33590, t33595, t33597, t33598, t33601)
}
