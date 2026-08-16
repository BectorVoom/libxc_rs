//! MGGA_C_TPSSLOC kxc pol — kxc_pol part 4 (v3rho3_2) CSE chunk 298/1228 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_kxc_pol_part4_v3rho3_2_chunk298<F: Float>(t300: F, t311: F, t890: F, t916: F, t919: F, t924: F, t933: F, t939: F, t943: F, t952: F, t315: F) -> (F, F, F) {
    let t956 = t300 * (-F::cast_from(0.310907e-1_f64) * t919 * t311 + F::cast_from(1.0_f64) * t924 * t933 + t890 - t916 - F::cast_from(0.19751673498613801407e-1_f64) * t939 + F::cast_from(0.5848223622634646207e0_f64) * t943 * t952);
    let t958 = F::cast_from(0.19751673498613801407e-1_f64) * t300 * t939;
    let t959 = t300 * t315;
    (t956, t958, t959)
}
