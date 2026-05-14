//! MGGA_C_PKZB lxc pol — lxc_pol part 10 (v4rho4_2) CSE chunk 1293/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part10_v4rho4_2_chunk1293<F: Float>(t1991: F, t9242: F, t1980: F, t3604: F, t5498: F, t730: F, t1281: F, t204: F, t3515: F) -> (F, F, F) {
    let t25622 = 0.11696447245269292414e1 * t9242 * t1991;
    let t25626 = 0.10389515463408878255e3 * t730 * t5498 * t3604 * t1980;
    let t25633 = t204 * t1281 * t3515;
    (t25622, t25626, t25633)
}
