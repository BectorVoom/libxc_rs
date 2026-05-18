//! MGGA_C_KCIS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 1211/1323 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part23_v4rho3sigma_5_chunk1211<F: Float>(t16623: F, t4288: F, t27529: F, t28640: F, t17334: F, t28624: F, t17446: F, t27544: F, t5916: F, t94748: F, t12265: F, t27543: F, t6012: F) -> (F, F, F, F, F, F) {
    let t97754 = t16623 * t4288;
    let t97756 = t28640 * t27529;
    let t97758 = t28624 * t17334;
    let t97760 = t27544 * t17446;
    let t97762 = t94748 * t5916;
    let t97765 = t12265 * t27543 * t6012;
    (t97754, t97756, t97758, t97760, t97762, t97765)
}
