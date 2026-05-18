//! MGGA_C_RMGGAC lxc pol — lxc_pol part 16 (v4rho3sigma_7) CSE chunk 493/1158 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part16_v4rho3sigma_7_chunk493<F: Float>(t209: F, t469: F, t6: F, t6247: F, t1468: F, t1508: F, t221: F, t1195: F, t1500: F, t4460: F, t4463: F, t4505: F, t4544: F, t467: F, t488: F, t5685: F, t5693: F, t5696: F, t6165: F, t6169: F, t6174: F, t6179: F, t6184: F, t6188: F, t6190: F, t6192: F, t6196: F, t6201: F, t6205: F, t6210: F, t6215: F) -> F {
    let t6250 = t469 * t6 * t6247 * t209;
    let t6254 = t221 * t1468 * t1508;
    let t6257 = F::new(0.10975822561044790898e0) * t1195 * t6165 + F::new(0.10975822561044790898e0) * t1195 * t6169 + F::new(0.54879112805223954488e-1) * t1195 * t6174 - F::new(0.27439556402611977244e-1) * t1500 * t6179 + F::new(0.54879112805223954488e-1) * t1195 * t6184 - F::new(0.42683754404063075713e0) * t5685 - t5693 + t5696 + F::new(0.64025631606094613569e-1) * t6188 + F::new(0.64025631606094613569e-1) * t6190 - F::new(0.12805126321218922714e0) * t6192 + F::new(0.16463733841567186346e0) * t488 * t6196 - F::new(0.65854935366268745384e0) * t488 * t6201 + F::new(0.32927467683134372692e0) * t488 * t6205 - F::new(0.42683754404063075712e0) * t4463 - t4460 - F::new(0.32927467683134372694e0) * t4505 * t6210 + F::new(0.10975822561044790898e0) * t1195 * t6215 - F::new(0.27439556402611977244e-1) * t467 * t6250 - F::new(0.21951645122089581796e0) * t4544 * t6254;
    t6257
}
