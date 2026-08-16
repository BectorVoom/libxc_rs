//! MGGA_C_KCISK lxc pol — lxc_pol part 3 (v3rho3_0) CSE chunk 536/1063 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_lxc_pol_part3_v3rho3_0_chunk536(t1537: f64, t4455: f64, t1527: f64, t507: f64, t515: f64, t4437: f64, t1197: f64, t1203: f64, t325: f64, t3696: f64, t1212: f64, t3697: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t4456 = t4455 * t1537;
    let t4459 = t1527 * t1527;
    let t4460 = 1.0_f64 / t4459;
    let t4461 = t507 * t4460;
    let t4462 = t515 * t515;
    let t4463 = 1.0_f64 / t4462;
    let t4464 = t4437 * t4463;
    let t4468 = t1197 * t1203;
    let t4471 = t325 * t3696;
    let t4472 = t3697 * t1212;
    (t4456, t4459, t4460, t4461, t4462, t4463, t4464, t4468, t4471, t4472)
}
