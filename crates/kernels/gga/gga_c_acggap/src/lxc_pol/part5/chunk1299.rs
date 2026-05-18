//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 1299/1332 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk1299<F: Float>(t1181: F, t12936: F, t4643: F, t5099: F, t1891: F, t3670: F, t1881: F, t3237: F, t1137: F, t6301: F, t6305: F, t3621: F, t6389: F) -> (F, F, F, F, F, F) {
    let t24145 = t12936 * t1181 * t4643 * t5099;
    let t24147 = t3670 * t1891;
    let t24149 = t3237 * t1881;
    let t24151 = t1137 * t6301;
    let t24153 = t1137 * t6305;
    let t24155 = t3621 * t6389;
    (t24145, t24147, t24149, t24151, t24153, t24155)
}
