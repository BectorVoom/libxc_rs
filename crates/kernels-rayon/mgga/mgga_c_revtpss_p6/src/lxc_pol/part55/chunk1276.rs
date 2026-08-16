//! MGGA_C_REVTPSS lxc pol — lxc_pol part 55 (v4rho2sigma2_10) CSE chunk 1276/1306 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part55_v4rho2sigma2_10_chunk1276(t125312: f64, t2121: f64, t2247: f64, t136: f64, t29411: f64, t8763: f64, t8995: f64, t196: f64, t197: f64, t29437: f64, t28166: f64, t1518: f64, t7583: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t129232 = t2247 * t125312 * t2121;
    let t129236 = t2247 * t29411 * t136;
    let t129353 = t8763 * t8995;
    let t129370 = t29437 * t196 * t197;
    let t129377 = t8763 * t28166;
    let t129467 = t7583 * t1518;
    (t129232, t129236, t129353, t129370, t129377, t129467)
}
