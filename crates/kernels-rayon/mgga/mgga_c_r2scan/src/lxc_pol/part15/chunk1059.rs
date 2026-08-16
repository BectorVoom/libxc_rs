//! MGGA_C_R2SCAN lxc pol — lxc_pol part 15 (v4rho3sigma_5) CSE chunk 1059/1253 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part15_v4rho3sigma_5_chunk1059(t37523: f64, t1266: f64, t2317: f64, t3446: f64, t3448: f64, t3434: f64, t3439: f64, t6860: f64, t875: f64, t10993: f64, t502: f64, t6876: f64) -> (f64, f64, f64, f64) {
    let t37524 = 0.63245127235888530833e-7_f64 * t37523;
    let t37527 = t3446 * t1266 * t2317 * t3448;
    let t37528 = 0.44715219694310041527e-2_f64 * t37527;
    let t37531 = t3434 * t6860 * t875 * t3439;
    let t37532 = 0.16432021104515675446e-2_f64 * t37531;
    let t37541 = t3446 * t502 * t6876 * t10993;
    (t37524, t37528, t37532, t37541)
}
