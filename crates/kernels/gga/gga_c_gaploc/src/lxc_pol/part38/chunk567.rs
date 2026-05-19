//! GGA_C_GAPLOC lxc pol — lxc_pol part 38 (v4rhosigma3_3) CSE chunk 567/1003 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part38_v4rhosigma3_3_chunk567<F: Float>(t10564: F, t10570: F, t10573: F, t10578: F, t10584: F, t10587: F, t10591: F, t10594: F, t10599: F, t10603: F, t10604: F, t1441: F, t1580: F, t1599: F, t1641: F, t193: F, t3372: F, t3387: F, t3403: F, t3415: F, t541: F, t557: F, t574: F, t597: F) -> F {
    let t10607 = F::cast_from(0.23833659967900284446e0_f64) * t3372 * t541 - F::cast_from(0.30674340763136599741e1_f64) * t574 * t10564 + F::cast_from(0.23005755572352449806e1_f64) * t1580 * t3415 + F::cast_from(0.23005755572352449806e1_f64) * t597 * t10570 + F::cast_from(0.30674340763136599741e1_f64) * t597 * t10573 - F::cast_from(0.35750489951850426669e0_f64) * t1599 * t3387 - F::cast_from(0.35750489951850426669e0_f64) * t557 * t10578 - F::cast_from(0.23005755572352449806e1_f64) * t1641 * t3403 - F::cast_from(0.23005755572352449806e1_f64) * t574 * t10584 + F::cast_from(0.35750489951850426669e0_f64) * t10587 * t193 + F::cast_from(0.35750489951850426669e0_f64) * t10591 * t193 - F::cast_from(0.23833659967900284446e0_f64) * t557 * t10594 + t10599 - t10603 + F::cast_from(0.51123901271894332902e0_f64) * t1441 * t10604;
    t10607
}
