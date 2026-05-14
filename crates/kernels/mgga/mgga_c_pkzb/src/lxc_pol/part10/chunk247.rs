//! MGGA_C_PKZB lxc pol — lxc_pol part 10 (v4rho4_2) CSE chunk 247/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part10_v4rho4_2_chunk247<F: Float>(t178: F, t774: F, t299: F, t208: F, t220: F) -> (F, F, F, F) {
    let t775 = t178 * t774;
    let t777 = 0.14291339372689912324e-3 * t299 * t775;
    let t778 = t220 * t208;
    let t779 = 1.0 / t778;
    (t775, t777, t778, t779)
}
