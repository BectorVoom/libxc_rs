//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 34 (v4rho3sigma_10) CSE chunk 873/1250 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part34_v4rho3sigma_10_chunk873(t20947: f64, t2701: f64, t820: f64, t20870: f64, t819: f64, t13283: f64, t1512: f64, t1516: f64, t16872: f64, t16976: f64, t20904: f64, t20908: f64, t20938: f64, t20944: f64, t249: f64, t4172: f64, t5587: f64, t5624: f64, t5628: f64, t817: f64, t843: f64, t9559: f64, t9974: f64) -> (f64, f64, f64) {
    let t20949 = t2701 * t820 * t20947;
    let t20953 = t819 * t820 * t20870;
    let t20958 = -t9974 * t20904 / 512.0_f64 - t843 * t20908 / 768.0_f64 + 5.0_f64 / 256.0_f64 * t4172 * t5624 - t16976 * t1516 / 256.0_f64 - t4172 * t5628 / 256.0_f64 + t20938 * t249 / 3072.0_f64 + t13283 * t5587 / 512.0_f64 - t9559 * t20944 / 4.0_f64 + 5.0_f64 / 256.0_f64 * t843 * t20949 - t817 * t20953 / 3072.0_f64 - t16872 * t1512 / 1024.0_f64;
    (t20949, t20953, t20958)
}
