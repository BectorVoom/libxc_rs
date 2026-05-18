//! GGA_C_ACGGAP lxc pol — lxc_pol part 12 (v4rho3sigma_4) CSE chunk 1112/1250 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part12_v4rho3sigma_4_chunk1112<F: Float>(t1562: F, t31824: F, t1449: F, t30148: F, t30159: F, t7586: F, t1541: F, t31611: F, t31350: F, t4921: F, t30219: F, t8473: F) -> (F, F, F, F, F) {
    let t35784 = t31824 * t1562;
    let t35788 = t30159 * t7586 * t30148 * t1449;
    let t35790 = t31611 * t1541;
    let t35792 = t31350 * t4921;
    let t35794 = t30219 * t8473;
    (t35784, t35788, t35790, t35792, t35794)
}
