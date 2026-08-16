//! MGGA_C_TPSSLOC kxc pol — kxc_pol part 4 (v3rho3_2) CSE chunk 1219/1228 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_kxc_pol_part4_v3rho3_2_chunk1219<F: Float>(t1375: F, t1386: F, t16022: F, t16460: F, t1843: F, t20038: F, t20040: F, t20044: F, t20048: F, t20051: F, t20060: F, t3758: F, t3882: F, t5215: F, t5326: F, t5354: F, t568: F, t6440: F, t6461: F) -> F {
    let t20062 = -F::cast_from(6.0_f64) * t1375 * t20051 - t1386 * t20044 - t1386 * t20060 - F::cast_from(2.0_f64) * t16022 * t1843 - F::cast_from(2.0_f64) * t16460 * t1843 + t20038 * t568 + t20040 * t568 + t20048 * t568 + F::cast_from(2.0_f64) * t3758 * t6440 - t3758 * t6461 + F::cast_from(2.0_f64) * t3882 * t6440 + F::cast_from(4.0_f64) * t5215 * t5326 - F::cast_from(2.0_f64) * t5215 * t5354;
    t20062
}
