//! MGGA_C_KCISK lxc pol — lxc_pol part 6 (v3rho3_3) CSE chunk 1080/1086 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_lxc_pol_part6_v3rho3_3_chunk1080(t31426: f64, t31450: f64, t31719: f64, t31742: f64, t2347: f64, t8436: f64, t8455: f64, t31212: f64, t31215: f64, t31218: f64, t31220: f64, t31223: f64, t31226: f64, t31229: f64, t31232: f64, t31235: f64, t31238: f64, t31241: f64, t31243: f64, t31245: f64, t31248: f64, t31250: f64, t31252: f64, t31254: f64, t31258: f64) -> (f64, f64, f64, f64) {
    let t31744 = t31426 + t31450 + t31719 + t31742;
    let t31752 = t8436 * t2347;
    let t31755 = t2347 * t8455;
    let t31776 = -0.101171875e-1_f64 * t31212 - 0.62499999999999999999e-1_f64 * t31215 - 0.80937499999999999999e-1_f64 * t31218 - 0.375e0_f64 * t31220 + 0.40468749999999999999e-1_f64 * t31223 - 0.13489583333333333333e-1_f64 * t31226 - 0.1875e0_f64 * t31229 - 0.60703125e-1_f64 * t31232 + 0.375e0_f64 * t31235 + 0.40468749999999999999e-1_f64 * t31238 - 0.1875e0_f64 * t31241 + 0.80937499999999999999e-1_f64 * t31243 - 0.40468749999999999999e-1_f64 * t31245 + 0.5625e0_f64 * t31248 - 0.28125e0_f64 * t31250 + 0.1875e0_f64 * t31252 + 0.303515625e-1_f64 * t31254 + 0.27777777777777777777e-1_f64 * t31258;
    (t31744, t31752, t31755, t31776)
}
