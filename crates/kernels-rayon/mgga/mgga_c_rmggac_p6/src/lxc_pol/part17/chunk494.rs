//! MGGA_C_RMGGAC lxc pol — lxc_pol part 17 (v4rho3sigma_8) CSE chunk 494/1111 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part17_v4rho3sigma_8_chunk494(t209: f64, t469: f64, t6: f64, t6247: f64, t1468: f64, t1508: f64, t221: f64, t1195: f64, t1500: f64, t4460: f64, t4463: f64, t4505: f64, t4544: f64, t467: f64, t488: f64, t5685: f64, t5693: f64, t5696: f64, t6165: f64, t6169: f64, t6174: f64, t6179: f64, t6184: f64, t6188: f64, t6190: f64, t6192: f64, t6196: f64, t6201: f64, t6205: f64, t6210: f64, t6215: f64) -> f64 {
    let t6250 = t469 * t6 * t6247 * t209;
    let t6254 = t221 * t1468 * t1508;
    let t6257 = 0.10975822561044790898e0_f64 * t1195 * t6165 + 0.10975822561044790898e0_f64 * t1195 * t6169 + 0.54879112805223954488e-1_f64 * t1195 * t6174 - 0.27439556402611977244e-1_f64 * t1500 * t6179 + 0.54879112805223954488e-1_f64 * t1195 * t6184 - 0.42683754404063075713e0_f64 * t5685 - t5693 + t5696 + 0.64025631606094613569e-1_f64 * t6188 + 0.64025631606094613569e-1_f64 * t6190 - 0.12805126321218922714e0_f64 * t6192 + 0.16463733841567186346e0_f64 * t488 * t6196 - 0.65854935366268745384e0_f64 * t488 * t6201 + 0.32927467683134372692e0_f64 * t488 * t6205 - 0.42683754404063075712e0_f64 * t4463 - t4460 - 0.32927467683134372694e0_f64 * t4505 * t6210 + 0.10975822561044790898e0_f64 * t1195 * t6215 - 0.27439556402611977244e-1_f64 * t467 * t6250 - 0.21951645122089581796e0_f64 * t4544 * t6254;
    t6257
}
