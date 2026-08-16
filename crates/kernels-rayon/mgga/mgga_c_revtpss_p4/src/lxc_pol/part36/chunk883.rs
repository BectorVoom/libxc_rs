//! MGGA_C_REVTPSS lxc pol — lxc_pol part 36 (v4rho3sigma_11) CSE chunk 883/1378 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part36_v4rho3sigma_11_chunk883(t1626: f64, t3011: f64, t1614: f64, t2967: f64, t2986: f64, t1596: f64, t2923: f64, t3090: f64, t4954: f64, t1646: f64, t3056: f64, t225: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t15350 = t1626 * t3011;
    let t15406 = t1614 * t2967;
    let t15413 = t1626 * t2986;
    let t15421 = t1596 * t2923;
    let t15618 = t4954 * t3090;
    let t15669 = t1646 * t3056;
    let t15670 = t15669 * t225;
    (t15350, t15406, t15413, t15421, t15618, t15669, t15670)
}
