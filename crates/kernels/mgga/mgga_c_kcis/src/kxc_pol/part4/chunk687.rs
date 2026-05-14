//! MGGA_C_KCIS kxc pol — kxc_pol part 4 (v3rho3_1) CSE chunk 687/1239 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_kxc_pol_part4_v3rho3_1_chunk687<F: Float>(t318: F, t565: F, t86: F, t1520: F, t3393: F, t1523: F, t238: F, t3751: F, t41: F, t3754: F, t538: F, t2642: F, t1455: F, t531: F, t1517: F, t833: F) -> (F, F, F, F, F, F, F) {
    let t4213 = 0.88437037037037037037e-2 * t86 * t318 * t565;
    let t4214 = t3393 * t1520;
    let t4217 = t86 * t238 * t1523;
    let t4219 = t41 * t3751;
    let t4220 = t538 * t3754;
    let t4222 = t4219 * t4220 * t2642;
    let t4225 = t1455 * t531;
    let t4227 = t1517 * t4225 * t833;
    (t4213, t4214, t4217, t4219, t4222, t4225, t4227)
}
