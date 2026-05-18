//! GGA_C_GAPC lxc pol — lxc_pol part 34 (v4rho2sigma2_13) CSE chunk 678/1427 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part34_v4rho2sigma2_13_chunk678<F: Float>(t144: F, t195: F, t102: F, t1946: F, t675: F, t681: F, t1: F, t567: F, t350: F, t505: F, t3712: F, t5054: F, t8: F) -> (F, F, F, F, F, F) {
    let t5581 = t195 * t144;
    let t5589 = t1946 * t102;
    let t5623 = t675 * t681;
    let t5624 = t567 * t1;
    let t5625 = t5624 * t350;
    let t5626 = t5623 * t5625;
    let t5631 = t505 * t1;
    let t5632 = t5631 * t350;
    let t5633 = t3712 * t5632;
    let t5658 = F::new(1.0) / t8 / t5054;
    (t5581, t5589, t5625, t5626, t5633, t5658)
}
