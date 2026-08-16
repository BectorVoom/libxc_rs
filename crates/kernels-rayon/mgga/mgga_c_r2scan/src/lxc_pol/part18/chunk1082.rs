//! MGGA_C_R2SCAN lxc pol — lxc_pol part 18 (v4rho3sigma_8) CSE chunk 1082/1264 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part18_v4rho3sigma_8_chunk1082(t38346: f64, t3428: f64, t3430: f64, t6836: f64, t10810: f64, t870: f64, t10684: f64, t10648: f64, t10958: f64, t10971: f64, t10962: f64, t11477: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t38347 = 0.91462949374725084942e-3_f64 * t38346;
    let t38349 = t6836 * t3428 * t3430;
    let t38350 = 0.15243824895787514157e-3_f64 * t38349;
    let t38355 = t870 * t10810;
    let t38356 = t38355 * t10684;
    let t38359 = t10648 * t10971 * t10958;
    let t38362 = t10648 * t10971 * t10962;
    let t38363 = 0.45731474687362542471e-3_f64 * t38362;
    let t39149 = 3.0_f64 / 2.0_f64 * t11477;
    (t38347, t38350, t38355, t38356, t38359, t38363, t39149)
}
