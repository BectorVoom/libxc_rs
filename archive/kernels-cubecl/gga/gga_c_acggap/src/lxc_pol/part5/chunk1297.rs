//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 1297/1332 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk1297<F: Float>(t3379: F, t5618: F, t1410: F, t2937: F, t406: F, t16899: F, t6324: F, t3409: F, t6086: F, t1101: F, t1165: F, t1889: F, t4282: F) -> (F, F, F, F, F) {
    let t24110 = t3379 * t5618;
    let t24112 = t2937 * t1410;
    let t24113 = t24112 * t406;
    let t24128 = t16899 * t6324;
    let t24130 = t3409 * t6086;
    let t24138 = t4282 * t1165 * t1889 * t1101;
    (t24110, t24113, t24128, t24130, t24138)
}
