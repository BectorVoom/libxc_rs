//! GGA_C_GAPLOC lxc pol — lxc_pol part 33 (v4rho2sigma2_16) CSE chunk 1202/1464 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part33_v4rho2sigma2_16_chunk1202<F: Float>(t32342: F, t10627: F, t161: F, t1845: F, t21488: F, t320: F, t795: F, t10701: F, t1841: F, t10632: F, t5524: F, t2925: F, t935: F) -> (F, F, F, F, F, F, F) {
    let t32343 = F::cast_from(0.32043859292259267849e-3_f64) * t32342;
    let t32348 = t10627 * t161;
    let t32349 = t32348 * t1845;
    let t32351 = F::cast_from(0.11963040802443459997e-1_f64) * t21488 * t320 * t795 * t32349;
    let t32352 = t1841 * t10701;
    let t32353 = F::cast_from(0.85450291446024714264e-3_f64) * t32352;
    let t32355 = F::cast_from(0.25635087433807414278e-2_f64) * t5524 * t10632;
    let t32356 = t2925 * t935;
    (t32343, t32348, t32349, t32351, t32353, t32355, t32356)
}
