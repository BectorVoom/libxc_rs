//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 29 (v4rho3sigma_5) CSE chunk 1918/2357 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1918<F: Float>(t26323: F, t6936: F, t22856: F, t22859: F, t22860: F, t22864: F, t22868: F, t26306: F, t26310: F, t26312: F, t26314: F, t26320: F) -> F {
    let t26324 = t6936 * t26323;
    let t26326 = t26306 / F::cast_from(384.0_f64) + t26310 / F::cast_from(768.0_f64) - t26312 / F::cast_from(1536.0_f64) + t26314 / F::cast_from(384.0_f64) + F::cast_from(0.33643963411783659045e-4_f64) * t22856 + t22859 - F::cast_from(7.0_f64) / F::cast_from(2304.0_f64) * t22860 + t22864 + t22868 + F::cast_from(0.40372756094140390854e-3_f64) * t26320 - F::cast_from(0.20186378047070195427e-3_f64) * t26324;
    t26326
}
