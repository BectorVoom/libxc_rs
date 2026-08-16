//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2462/3221 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2462<F: Float>(t11282: F, t1143: F, t43689: F, t440: F, t43776: F, t43819: F, t3324: F, t3356: F, t3330: F, t3355: F, t427: F, t1174: F, t3471: F, t698: F) -> (F, F, F, F, F, F, F, F, F) {
    let t44220 = t1143 * t11282;
    let t44223 = t440 * t43689;
    let t44249 = F::cast_from(0.16979925925925925926e1_f64) * t43776;
    let t44275 = F::cast_from(0.5356037037037037037e1_f64) * t43819;
    let t44300 = t3324 * t3356;
    let t44320 = F::cast_from(0.17757530864197530864e0_f64) * t43819;
    let t44348 = F::cast_from(0.18467901234567901234e0_f64) * t43819;
    let t44361 = t427 / t3355 / t3330;
    let t44424 = t1174 * t698 * t3471;
    (t44220, t44223, t44249, t44275, t44300, t44320, t44348, t44361, t44424)
}
