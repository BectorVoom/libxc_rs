//! MGGA_C_KCIS lxc pol — lxc_pol part 4 (v3rho3_1) CSE chunk 636/1420 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part4_v3rho3_1_chunk636(t3463: f64, t3228: f64, t359: f64, t376: f64, t1166: f64, t1176: f64, t1180: f64, t1094: f64, t1164: f64, t1172: f64, t3177: f64, t381: f64, sigma0: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t3464 = t3463 * sigma0;
    let t3465 = t359 * t3228;
    let t3466 = t376 * t3465;
    let t3467 = t3464 * t3466;
    let t3469 = t1166 * t1176;
    let t3471 = t1166 * t1180;
    let t3473 = t1164 * t1094;
    let t3474 = t3473 * sigma0;
    let t3475 = t3474 * t1172;
    let t3477 = t3177 * t381;
    (t3466, t3467, t3469, t3471, t3473, t3474, t3475, t3477)
}
