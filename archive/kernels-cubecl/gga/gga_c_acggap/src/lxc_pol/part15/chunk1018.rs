//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 1018/1278 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk1018<F: Float>(t1449: F, t30148: F, t30159: F, t7586: F, t1541: F, t31611: F, t30219: F, t8473: F, t4680: F, t7426: F, t8605: F, t30468: F, t4916: F) -> (F, F, F, F, F) {
    let t35788 = t30159 * t7586 * t30148 * t1449;
    let t35790 = t31611 * t1541;
    let t35794 = t30219 * t8473;
    let t35797 = t7426 * t4680 * t8605;
    let t35799 = t30468 * t4916;
    (t35788, t35790, t35794, t35797, t35799)
}
