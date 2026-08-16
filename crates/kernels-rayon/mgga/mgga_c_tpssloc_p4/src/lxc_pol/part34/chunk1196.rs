//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 34 (v4rho3sigma_10) CSE chunk 1196/1250 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part34_v4rho3sigma_10_chunk1196(t1339: f64, t1799: f64, t22827: f64, t6420: f64, t1825: f64, t6414: f64, t6936: f64, t16311: f64, t3788: f64, t20554: f64, t20563: f64, t221: f64, t26284: f64) -> (f64, f64, f64, f64, f64) {
    let t107174 = t22827 * t1339 * t6420 * t1799;
    let t107178 = t6936 * t1339 * t1825 * t6414;
    let t107183 = t6936 * t3788 * t16311 * t6414;
    let t107186 = t6936 * t1339 * t20554;
    let t107189 = t26284 * t221 * t20563;
    (t107174, t107178, t107183, t107186, t107189)
}
