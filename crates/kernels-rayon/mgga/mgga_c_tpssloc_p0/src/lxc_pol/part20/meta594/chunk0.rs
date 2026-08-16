//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2173/2712 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2173(t11282: f64, t1143: f64, t43689: f64, t440: f64, t43776: f64, t43819: f64, t1128: f64, t11455: f64, t3324: f64, t3356: f64, t3330: f64, t3355: f64, t427: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t44220 = t1143 * t11282;
    let t44223 = t440 * t43689;
    let t44249 = 0.16979925925925925926e1_f64 * t43776;
    let t44275 = 0.5356037037037037037e1_f64 * t43819;
    let t44295 = t11455 * t1128;
    let t44300 = t3324 * t3356;
    let t44320 = 0.17757530864197530864e0_f64 * t43819;
    let t44348 = 0.18467901234567901234e0_f64 * t43819;
    let t44361 = t427 / t3355 / t3330;
    (t44220, t44223, t44249, t44275, t44295, t44300, t44320, t44348, t44361)
}
