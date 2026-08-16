//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 604/1340 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk604<F: Float>(t27: F, t3333: F, t23: F, t28: F, t3315: F, t3319: F, t3324: F, t3330: F, t7: F, t980: F, t984: F) -> (F, F) {
    let t3334 = t27 * t3333;
    let t3337 = F::cast_from(10.0_f64) / F::cast_from(9.0_f64) * t7 * t3315 + F::cast_from(5.0_f64) / F::cast_from(3.0_f64) * t7 * t3319 + F::cast_from(88.0_f64) / F::cast_from(9.0_f64) * t3324 * t28 - F::cast_from(80.0_f64) / F::cast_from(9.0_f64) * t980 * t984 + F::cast_from(10.0_f64) / F::cast_from(9.0_f64) * t23 * t3330 + F::cast_from(5.0_f64) / F::cast_from(3.0_f64) * t23 * t3334;
    (t3334, t3337)
}
