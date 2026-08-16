//! MGGA_C_RMGGAC lxc pol — lxc_pol part 12 (v4rho3sigma_3) CSE chunk 898/1088 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part12_v4rho3sigma_3_chunk898(t34847: f64, t8831: f64, t1550: f64, t5144: f64, t7778: f64, t2060: f64, t27177: f64, t4044: f64, t289: f64, t35124: f64, t35128: f64, t35130: f64, t35132: f64, t39491: f64, t39493: f64, t39495: f64, t39497: f64, t39499: f64, t39506: f64, t39507: f64, t39514: f64, t39518: f64, t39523: f64) -> f64 {
    let t39525 = t34847 * t8831;
    let t39528 = t1550 * t7778 * t5144;
    let t39529 = 0.15965655602485078085e0_f64 * t39528;
    let t39531 = t4044 * t2060 * t27177;
    let t39533 = -0.76616279807936110914e-4_f64 * t39491 - 0.25538759935978703638e-4_f64 * t39493 + 0.25538759935978703638e-4_f64 * t39495 + 0.85129199786595678796e-5_f64 * t39497 + 0.1064114997332445985e-4_f64 * t39499 - 0.15243824895787514157e-3_f64 * t35124 + 0.21684485328539747656e-4_f64 * t35128 - 0.90915538847484472429e-2_f64 * t35130 + 0.15965655602485078085e0_f64 * t35132 - t39506 - 0.4726e1_f64 * t289 * t39507 - 0.85129199786595678796e-5_f64 * t39514 - 0.85129199786595678796e-5_f64 * t39518 + 0.53205749866622299248e-5_f64 * t39523 - 0.31923449919973379548e-4_f64 * t39525 - t39529 + 0.17961362552795712846e0_f64 * t39531;
    t39533
}
