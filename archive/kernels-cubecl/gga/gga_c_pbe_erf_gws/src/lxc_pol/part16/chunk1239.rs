//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 16 (v4rho3sigma_4) CSE chunk 1239/1361 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part16_v4rho3sigma_4_chunk1239<F: Float>(t3038: F, t3972: F, t3975: F, t9520: F, t1176: F, t14639: F, t6365: F, t923: F, t3959: F, t8766: F, t1113: F, t28647: F) -> (F, F, F, F) {
    let t53395 = t3972 * t3975 * t3038 * t9520;
    let t53424 = t1176 * t923 * t6365 * t14639;
    let t53426 = t3959 * t8766;
    let t53432 = t3972 * t3975 * t1113 * t28647;
    (t53395, t53424, t53426, t53432)
}
