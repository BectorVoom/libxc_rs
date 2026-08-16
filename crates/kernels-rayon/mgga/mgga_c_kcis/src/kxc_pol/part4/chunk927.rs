//! MGGA_C_KCIS kxc pol — kxc_pol part 4 (v3rho3_1) CSE chunk 927/1420 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_kxc_pol_part4_v3rho3_1_chunk927(t187: f64, t2709: f64, t8631: f64, t8634: f64, t8637: f64, t867: f64, t8682: f64, t8700: f64, t8704: f64, t8708: f64, t8713: f64, t8725: f64, t8737: f64, t8745: f64, t8849: f64, t8893: f64) -> f64 {
    let t8912 = t187 * (t8849 + t8893) - 0.51947267698127589897e2_f64 * t867 * t8713 + 0.1038945353962551798e3_f64 * t867 * t8682 - 0.58482233974552040708e0_f64 * t867 * t8700 - 0.21687161765563048428e-1_f64 * t2709 * t8634 + 0.16265371324172286321e-1_f64 * t2709 * t8637 - t8725 + t8737 + t8745 - 0.35089340384731224426e1_f64 * t867 * t8704 + 0.35089340384731224426e1_f64 * t867 * t8708 - 0.32530742648344572643e-1_f64 * t2709 * t8631;
    t8912
}
