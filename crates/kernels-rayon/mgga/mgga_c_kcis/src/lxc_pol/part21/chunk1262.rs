//! MGGA_C_KCIS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 1262/1389 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part21_v4rho3sigma_3_chunk1262(t26941: f64, t28045: f64, t1796: f64, t982: f64, t7755: f64, t26924: f64, t5078: f64, t26929: f64, t5025: f64, t3439: f64, t14788: f64, t7754: f64) -> (f64, f64, f64, f64, f64) {
    let t95374 = t28045 * t26941;
    let t95376 = t1796 * t982;
    let t95377 = t95376 * t7755;
    let t95379 = t26924 * t5078;
    let t95381 = t5025 * t26929;
    let t95382 = t95381 * t3439;
    let t95384 = t7754 * t14788;
    (t95374, t95377, t95379, t95382, t95384)
}
