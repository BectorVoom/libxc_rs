//! MGGA_C_KCIS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 279/1397 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part26_v4rho3sigma_8_chunk279<F: Float>(t1360: F, t143: F, t238: F, t565: F, t86: F, t1392: F, t41: F) -> (F, F, F) {
    let t1507 = t1360 * t143;
    let t1516 = F::new(0.26531111111111111111e-1) * t86 * t238 * t565;
    let t1517 = t41 * t1392;
    (t1507, t1516, t1517)
}
