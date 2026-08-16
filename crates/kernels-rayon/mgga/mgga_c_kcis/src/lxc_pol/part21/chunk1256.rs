//! MGGA_C_KCIS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 1256/1389 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part21_v4rho3sigma_3_chunk1256(t14735: f64, t26930: f64, t5099: f64, t92515: f64, t15086: f64, t28024: f64, t26938: f64, t5078: f64, t26933: f64, t5062: f64, t14778: f64, t7748: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t95305 = t26930 * t14735;
    let t95307 = t92515 * t5099;
    let t95309 = t28024 * t15086;
    let t95311 = t26938 * t5078;
    let t95313 = t26933 * t5062;
    let t95315 = t7748 * t14778;
    (t95305, t95307, t95309, t95311, t95313, t95315)
}
