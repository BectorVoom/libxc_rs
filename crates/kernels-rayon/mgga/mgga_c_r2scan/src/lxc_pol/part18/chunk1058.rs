//! MGGA_C_R2SCAN lxc pol — lxc_pol part 18 (v4rho3sigma_8) CSE chunk 1058/1264 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part18_v4rho3sigma_8_chunk1058(t37527: f64, t3434: f64, t3439: f64, t6860: f64, t875: f64, t10993: f64, t3446: f64, t502: f64, t6876: f64, t10954: f64, t10958: f64, t10962: f64) -> (f64, f64, f64, f64, f64) {
    let t37528 = 0.44715219694310041527e-2_f64 * t37527;
    let t37531 = t3434 * t6860 * t875 * t3439;
    let t37532 = 0.16432021104515675446e-2_f64 * t37531;
    let t37541 = t3446 * t502 * t6876 * t10993;
    let t37542 = 0.24390119833260022651e-2_f64 * t37541;
    let t37556 = t3446 * t10954 * t10958;
    let t37560 = t3446 * t10954 * t10962;
    (t37528, t37532, t37542, t37556, t37560)
}
