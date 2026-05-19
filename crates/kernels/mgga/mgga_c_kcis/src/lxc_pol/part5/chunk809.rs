//! MGGA_C_KCIS lxc pol — lxc_pol part 5 (v3rho3_2) CSE chunk 809/1419 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part5_v3rho3_2_chunk809<F: Float>(t6389: F, t950: F, t931: F, t2988: F, t6365: F, t2986: F, t2992: F, t4612: F, t6328: F, t6332: F, t6336: F, t274: F) -> (F, F, F, F, F, F) {
    let t6390 = t6389 * t950;
    let t6392 = F::new(1.0) * t931 * t6390;
    let t6393 = t6365 * t2988;
    let t6395 = F::cast_from(0.16081824322151104822e2_f64) * t2986 * t6393;
    let t6400 = t2992 + F::cast_from(0.61805555555555555556e-2_f64) * t4612 - F::cast_from(0.61805555555555555555e-2_f64) * t6328 + F::cast_from(0.18541666666666666667e-1_f64) * t6332 - F::cast_from(0.92708333333333333333e-2_f64) * t6336;
    let t6401 = t6400 * t274;
    (t6390, t6392, t6393, t6395, t6400, t6401)
}
