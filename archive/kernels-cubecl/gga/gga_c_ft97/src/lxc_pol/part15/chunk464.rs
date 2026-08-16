//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 464/1222 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk464<F: Float>(t2179: F, t4724: F, t144: F, t167: F, t2185: F, t4668: F, t1017: F, t1053: F) -> (F, F, F, F) {
    let t4725 = t2179 * t4724;
    let t4726 = t144 * t4725;
    let t4730 = t2185 * t167 * t4668;
    let t4733 = t1017 * t1053;
    (t4725, t4726, t4730, t4733)
}
