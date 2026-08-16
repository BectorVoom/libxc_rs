//! MGGA_C_RMGGAC lxc pol — lxc_pol part 13 (v4rho3sigma_4) CSE chunk 877/1127 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part13_v4rho3sigma_4_chunk877<F: Float>(t7501: F, t8562: F, t2139: F, t27: F, t4928: F, t649: F, t1605: F, t1986: F, t7720: F, t36787: F, t8571: F, t35559: F) -> (F, F, F, F, F) {
    let t39482 = t7501 * t8562;
    let t39486 = t2139 * t27 * t649 * t4928;
    let t39490 = t1986 * t1605;
    let t39491 = t7720 * t39490;
    let t39493 = t8571 * t36787;
    let t39495 = t8571 * t35559;
    (t39482, t39486, t39491, t39493, t39495)
}
