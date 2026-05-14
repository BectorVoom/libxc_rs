//! GGA_C_FT97 kxc pol — kxc_pol part 3 (v3rho3_2) CSE chunk 534/887 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_kxc_pol_part3_v3rho3_2_chunk534<F: Float>(t2179: F, t4724: F, t144: F, t167: F, t2185: F, t4668: F, t1017: F, t1053: F, t574: F, t605: F, t1060: F, t569: F, t925: F, t4462: F, t2205: F, t4454: F) -> (F, F, F, F, F, F, F, F) {
    let t4725 = t2179 * t4724;
    let t4726 = t144 * t4725;
    let t4730 = t2185 * t167 * t4668;
    let t4733 = t1017 * t1053;
    let t4735 = t574 * t605 * t4733;
    let t4739 = t569 * t1060 * t925;
    let t4743 = t569 * t167 * t4462;
    let t4747 = t2205 * t167 * t4454;
    (t4725, t4726, t4730, t4733, t4735, t4739, t4743, t4747)
}
