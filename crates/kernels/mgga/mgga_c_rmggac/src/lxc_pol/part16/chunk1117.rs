//! MGGA_C_RMGGAC lxc pol — lxc_pol part 16 (v4rho3sigma_7) CSE chunk 1117/1158 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part16_v4rho3sigma_7_chunk1117<F: Float>(t49048: F, t49064: F, t49082: F, t49095: F, t49110: F, t49126: F, t49143: F, t49151: F, t10484: F, t2211: F, t235: F, t2435: F, t30221: F, t30283: F, t36521: F, t37976: F, t43783: F, t43784: F, t43792: F, t47405: F, t47408: F, t47410: F, t47414: F, t47417: F, t47429: F, t47432: F, t504: F, t515: F, t623: F, t6557: F, t8264: F, t884: F, t9487: F) -> (F, F) {
    let t49154 = t49048 + t49064 + t49082 + t49095 + t49110 + t49126 + t49143 + t49151;
    let t49175 = -F::cast_from(0.82764499792523576609e-4_f64) * t36521 - F::cast_from(0.85129199786595678799e-5_f64) * t47405 + t43783 - t43784 - F::cast_from(0.11974241701863808564e0_f64) * t47408 - F::cast_from(0.19957069503106347607e-1_f64) * t235 * t515 * t49154 + F::cast_from(0.5987120850931904282e-1_f64) * t47410 + t37976 - F::cast_from(0.55866037359953414211e-4_f64) * t47414 - F::cast_from(0.11974241701863808564e0_f64) * t47417 + t43792 + F::cast_from(0.79828278012425390428e-1_f64) * t30221 * t2435 - F::cast_from(0.19957069503106347607e-1_f64) * t504 * t10484 + F::cast_from(0.212822999466489197e-4_f64) * t47429 - F::cast_from(0.23948483403727617128e0_f64) * t884 * t8264 * t6557 - F::cast_from(0.23948483403727617128e0_f64) * t884 * t2211 * t30283 - F::cast_from(0.39914139006212695214e-1_f64) * t623 * t9487 - F::cast_from(0.30487649791575028312e-3_f64) * t47432;
    (t49154, t49175)
}
