//! MGGA_C_RMGGAC lxc pol — lxc_pol part 14 (v4rho3sigma_5) CSE chunk 264/1089 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part14_v4rho3sigma_5_chunk264(t1228: f64, t492: f64, t140: f64, t453: f64, t1144: f64, t490: f64, t489: f64, t998: f64, t1181: f64, t1186: f64, t1191: f64, t1195: f64, t1198: f64, t1215: f64, t1219: f64, t1227: f64, t467: f64, t488: f64) -> (f64, f64, f64, f64, f64) {
    let t1229 = t1228 * t492;
    let t1231 = t453 * t140;
    let t1233 = t1231 * t490 * t1144;
    let t1237 = t489 * t490 * t998;
    let t1240 = 0.54879112805223954488e-1_f64 * t1181 * t1186 + 0.12805126321218922714e0_f64 * t1191 + 0.10975822561044790898e0_f64 * t1195 * t1198 - 0.27439556402611977244e-1_f64 * t467 * t1215 - 0.27439556402611977244e-1_f64 * t467 * t1219 + t1227 + 0.25610252642437845428e0_f64 * t1229 + 0.16463733841567186346e0_f64 * t488 * t1233 - 0.54879112805223954488e-1_f64 * t488 * t1237;
    (t1229, t1231, t1233, t1237, t1240)
}
