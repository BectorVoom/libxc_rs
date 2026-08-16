//! MGGA_C_KCIS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 736/1397 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part26_v4rho3sigma_8_chunk736<F: Float>(t7952: F, t8196: F, t5752: F, t585: F, t1468: F, t2055: F, t1395: F, t2062: F, t2066: F, t8189: F, t8192: F, t8194: F) -> (F, F, F, F, F, F) {
    let t8197 = t7952 * t8196;
    let t8199 = t5752 * t585;
    let t8201 = t1468 * t2055;
    let t8203 = t1395 * t2062;
    let t8205 = t1395 * t2066;
    let t8207 = t8189 / F::cast_from(16.0_f64) - t8192 / F::cast_from(16.0_f64) - t8194 / F::cast_from(6.0_f64) + t8197 / F::cast_from(24.0_f64) - t8199 / F::cast_from(128.0_f64) + t8201 / F::cast_from(128.0_f64) + t8203 / F::cast_from(24.0_f64) - t8205 / F::cast_from(96.0_f64);
    (t8197, t8199, t8201, t8203, t8205, t8207)
}
