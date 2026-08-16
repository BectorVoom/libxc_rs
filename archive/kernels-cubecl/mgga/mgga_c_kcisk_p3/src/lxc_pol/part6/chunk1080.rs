//! MGGA_C_KCISK lxc pol — lxc_pol part 6 (v3rho3_3) CSE chunk 1080/1086 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part6_v3rho3_3_chunk1080<F: Float>(t31426: F, t31450: F, t31719: F, t31742: F, t2347: F, t8436: F, t8455: F, t31212: F, t31215: F, t31218: F, t31220: F, t31223: F, t31226: F, t31229: F, t31232: F, t31235: F, t31238: F, t31241: F, t31243: F, t31245: F, t31248: F, t31250: F, t31252: F, t31254: F, t31258: F) -> (F, F, F, F) {
    let t31744 = t31426 + t31450 + t31719 + t31742;
    let t31752 = t8436 * t2347;
    let t31755 = t2347 * t8455;
    let t31776 = -F::cast_from(0.101171875e-1_f64) * t31212 - F::cast_from(0.62499999999999999999e-1_f64) * t31215 - F::cast_from(0.80937499999999999999e-1_f64) * t31218 - F::cast_from(0.375e0_f64) * t31220 + F::cast_from(0.40468749999999999999e-1_f64) * t31223 - F::cast_from(0.13489583333333333333e-1_f64) * t31226 - F::cast_from(0.1875e0_f64) * t31229 - F::cast_from(0.60703125e-1_f64) * t31232 + F::cast_from(0.375e0_f64) * t31235 + F::cast_from(0.40468749999999999999e-1_f64) * t31238 - F::cast_from(0.1875e0_f64) * t31241 + F::cast_from(0.80937499999999999999e-1_f64) * t31243 - F::cast_from(0.40468749999999999999e-1_f64) * t31245 + F::cast_from(0.5625e0_f64) * t31248 - F::cast_from(0.28125e0_f64) * t31250 + F::cast_from(0.1875e0_f64) * t31252 + F::cast_from(0.303515625e-1_f64) * t31254 + F::cast_from(0.27777777777777777777e-1_f64) * t31258;
    (t31744, t31752, t31755, t31776)
}
