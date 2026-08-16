//! MGGA_C_REVTPSS lxc pol — lxc_pol part 56 (v4rho2sigma2_11) CSE chunk 962/1203 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part56_v4rho2sigma2_11_chunk962(t1263: f64, t494: f64, t1122: f64, t32015: f64, t1276: f64, t1294: f64, t247: f64, t3719: f64, t1209: f64, t8931: f64, t7642: f64, t2142: f64, t2148: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t33426 = t1263 * t494;
    let t33427 = t33426 * t1122;
    let t33428 = t32015 * t33427;
    let t33431 = t1276 * t1294;
    let t33433 = t247 * t3719 * t33431;
    let t33436 = t1209 * t8931;
    let t33441 = t7642 * t8931;
    let t33446 = t2148 * t2142;
    (t33426, t33427, t33428, t33431, t33433, t33436, t33441, t33446)
}
