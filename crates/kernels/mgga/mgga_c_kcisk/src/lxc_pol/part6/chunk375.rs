//! MGGA_C_KCISK lxc pol — lxc_pol part 6 (v3rho3_3) CSE chunk 375/957 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part6_v3rho3_3_chunk375<F: Float>(t1819: F, t2366: F, t2373: F, t1815: F, t2372: F, t574: F) -> (F, F) {
    let t2484 = 0.1982e-1 * t2373 - t1819 - 0.41275e-2 * t2366;
    let t2487 = t1815 * t2372 / 4.0 + t574 * t2484 / 2.0;
    (t2484, t2487)
}
