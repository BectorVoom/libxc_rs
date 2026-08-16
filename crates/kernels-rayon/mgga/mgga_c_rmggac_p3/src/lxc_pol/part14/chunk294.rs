//! MGGA_C_RMGGAC lxc pol — lxc_pol part 14 (v4rho3sigma_5) CSE chunk 294/1089 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part14_v4rho3sigma_5_chunk294(t1231: f64, t219: f64, t589: f64, t6: f64, t446: f64, t1392: f64, t489: f64, t490: f64, t1191: f64, t1195: f64, t1227: f64, t1229: f64, t1467: f64, t1470: f64, t1473: f64, t1477: f64, t1497: f64, t1500: f64, t1503: f64, t1510: f64, t1513: f64, t467: f64, t488: f64) -> (f64, f64, f64, f64, f64) {
    let t1515 = t1231 * t219;
    let t1516 = t6 * t589;
    let t1518 = t1515 * t1516 * t446;
    let t1522 = t489 * t490 * t1392;
    let t1525 = 0.54879112805223954488e-1_f64 * t1467 * t1470 + 0.64025631606094613569e-1_f64 * t1473 + 0.54879112805223954488e-1_f64 * t1195 * t1477 - 0.27439556402611977244e-1_f64 * t467 * t1497 - 0.27439556402611977244e-1_f64 * t1500 * t1503 + 0.64025631606094613569e-1_f64 * t1191 + t1227 + 0.12805126321218922714e0_f64 * t1229 + 0.54879112805223954488e-1_f64 * t1195 * t1510 + 0.12805126321218922714e0_f64 * t1513 + 0.16463733841567186346e0_f64 * t488 * t1518 - 0.54879112805223954488e-1_f64 * t488 * t1522;
    (t1515, t1516, t1518, t1522, t1525)
}
