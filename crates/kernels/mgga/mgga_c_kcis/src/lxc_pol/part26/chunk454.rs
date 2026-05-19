//! MGGA_C_KCIS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 454/1397 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part26_v4rho3sigma_8_chunk454<F: Float>(t3793: F, t3879: F, t1346: F) -> (F, F, F, F) {
    let t3926 = F::cast_from(0.40256666666666666667e0_f64) * t3793;
    let t3933 = F::new(0.137975e0) * t3879;
    let t3943 = t1346 * t1346;
    let t3944 = F::new(1.0) / t3943;
    (t3926, t3933, t3943, t3944)
}
