//! MGGA_C_KCIS lxc pol — lxc_pol part 4 (v3rho3_1) CSE chunk 629/1239 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part4_v3rho3_1_chunk629<F: Float>(t3754: F, t544: F, t2642: F, t3752: F, t1392: F, t456: F) -> (F, F) {
    let t3755 = t544 * t3754;
    let t3757 = t3752 * t3755 * t2642;
    let t3760 = t1392 * t456;
    let t3761 = t3760 * t544;
    (t3757, t3761)
}
