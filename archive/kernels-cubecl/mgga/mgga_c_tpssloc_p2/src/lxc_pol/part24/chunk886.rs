//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 24 (v4rho3sigma_0) CSE chunk 886/1438 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part24_v4rho3sigma_0_chunk886<F: Float>(t252: F, t2678: F, t829: F, t860: F, t9661: F, t10016: F, t10055: F, t10058: F, t10069: F, t10073: F, t10077: F, t10081: F, t10084: F, t10091: F, t10094: F, t226: F, t255: F, t2613: F, t2617: F, t2729: F, t2733: F, t2736: F, t2738: F, t2740: F, t4281: F, t4291: F, t808: F, t812: F, t861: F, t863: F, t9612: F) -> (F, F, F) {
    let t10097 = t252 * t2678;
    let t10098 = t10097 * t829;
    let t10101 = t860 * t9661;
    let t10103 = t10016 * t255 + F::cast_from(6.0_f64) * t10055 * t812 + t10058 * t226 - t10069 * t812 - F::cast_from(3.0_f64) * t10073 * t812 - F::cast_from(3.0_f64) * t10077 * t812 - F::cast_from(6.0_f64) * t10081 * t812 + F::cast_from(6.0_f64) * t10084 * t812 - F::cast_from(3.0_f64) * t10091 * t812 + F::cast_from(6.0_f64) * t10094 * t4281 - F::cast_from(3.0_f64) * t10098 * t4291 - t10101 * t812 + F::cast_from(3.0_f64) * t2613 * t863 + F::cast_from(6.0_f64) * t2617 * t2729 - F::cast_from(6.0_f64) * t2617 * t2733 - F::cast_from(3.0_f64) * t2617 * t2736 - F::cast_from(3.0_f64) * t2617 * t2738 + F::cast_from(3.0_f64) * t2740 * t808 - F::cast_from(3.0_f64) * t861 * t9612;
    (t10097, t10098, t10103)
}
