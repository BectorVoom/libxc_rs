//! MGGA_C_KCIS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 1034/1243 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part26_v4rho3sigma_8_chunk1034<F: Float>(t1444: F, t1616: F, t5654: F, t6159: F, t3754: F) -> (F, F, F, F) {
    let t28758 = t1616 * t1444;
    let t28759 = t28758 * t5654;
    let t28760 = t6159 * t28759;
    let t28765 = t1616 * t3754;
    (t28758, t28759, t28760, t28765)
}
