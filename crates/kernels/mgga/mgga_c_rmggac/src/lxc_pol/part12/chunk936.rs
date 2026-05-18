//! MGGA_C_RMGGAC lxc pol — lxc_pol part 12 (v4rho3sigma_3) CSE chunk 936/1088 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part12_v4rho3sigma_3_chunk936<F: Float>(t38530: F, t7469: F, t3351: F, t3352: F, t5163: F, t880: F, t2144: F, t5166: F, t2412: F, t7682: F, t1990: F, t9087: F) -> (F, F, F, F, F) {
    let t40102 = t38530 * t7469;
    let t40106 = t3351 * t3352 * t880 * t5163;
    let t40110 = t3351 * t3352 * t2144 * t5166;
    let t40112 = t2412 * t7682;
    let t40114 = t9087 * t1990;
    (t40102, t40106, t40110, t40112, t40114)
}
