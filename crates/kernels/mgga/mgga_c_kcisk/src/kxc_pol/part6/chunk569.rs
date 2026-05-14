//! MGGA_C_KCISK kxc pol — kxc_pol part 6 (v3rho3_3) CSE chunk 569/957 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_kxc_pol_part6_v3rho3_3_chunk569<F: Float>(t2063: F, t2441: F, t5193: F, t5192: F, t5182: F, t682: F, t7715: F, t2487: F, t6734: F, t7718: F) -> (F, F, F, F, F, F, F) {
    let t8484 = t2063 * t2441;
    let t8485 = t5193 * t8484;
    let t8486 = t5192 * t8485;
    let t8487 = t5182 * t8486;
    let t8491 = t682 * t7715;
    let t8494 = t6734 * t2487;
    let t8497 = t682 * t7718;
    let t8500 = t2487 * t2487;
    (t8485, t8486, t8487, t8491, t8494, t8497, t8500)
}
