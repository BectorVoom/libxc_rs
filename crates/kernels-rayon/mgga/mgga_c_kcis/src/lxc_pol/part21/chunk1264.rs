//! MGGA_C_KCIS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 1264/1389 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part21_v4rho3sigma_3_chunk1264(t26929: f64, t9588: f64, t14850: f64, t8072: f64, t92532: f64, t26891: f64, t5091: f64, t14812: f64, t28029: f64, t1176: f64, t5164: f64, t26933: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t95391 = t9588 * t26929;
    let t95392 = t95391 * t14850;
    let t95394 = t92532 * t8072;
    let t95396 = t26891 * t5091;
    let t95398 = t28029 * t14812;
    let t95400 = t5164 * t1176;
    let t95402 = t26933 * t5091;
    (t95392, t95394, t95396, t95398, t95400, t95402)
}
