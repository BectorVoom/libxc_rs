//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 1411/2712 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1411<F: Float>(t10186: F, t10192: F, t10196: F, t10200: F, t10204: F, t10209: F, t10219: F, t10226: F, t10229: F, t10233: F, t10238: F, t10242: F, t10246: F, t10251: F, t10256: F, t10260: F, t10263: F, t2960: F, t2982: F, t2986: F, t2991: F, t973: F, t980: F) -> F {
    let t10266 = F::cast_from(0.44444444444444444443e-2_f64) * t10186 * t2991 - F::cast_from(0.55555555555555555554e-3_f64) * t10192 + F::cast_from(0.11111111111111111111e-2_f64) * t2986 * t10196 + F::cast_from(0.16666666666666666666e-2_f64) * t973 * t10200 + F::cast_from(0.27777777777777777777e-3_f64) * t973 * t10204 - F::cast_from(0.24999999999999999999e-2_f64) * t973 * t10209 + F::cast_from(0.86419753086419753084e-3_f64) * t973 * t10219 - F::cast_from(0.29629629629629629629e-2_f64) * t2960 * t2982 - F::cast_from(0.18518518518518518518e-3_f64) * t10226 + F::cast_from(0.27777777777777777777e-3_f64) * t10229 + F::cast_from(0.37037037037037037036e-3_f64) * t10233 - F::cast_from(0.11111111111111111111e-2_f64) * t2986 * t10238 - F::cast_from(0.83333333333333333331e-3_f64) * t2986 * t10242 - F::cast_from(0.83333333333333333331e-3_f64) * t2986 * t10246 - F::cast_from(0.16666666666666666666e-2_f64) * t2986 * t10251 + F::cast_from(0.16666666666666666666e-2_f64) * t2986 * t10256 - F::cast_from(0.83333333333333333331e-3_f64) * t2986 * t10260 + F::cast_from(0.81481481481481481478e-2_f64) * t10263 * t980;
    t10266
}
