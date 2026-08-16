//! MGGA_C_KCIS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 1191/1323 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part23_v4rho3sigma_5_chunk1191(t7908: f64, t94585: f64, t27484: f64, t7895: f64, t1014: f64, t27332: f64, t27424: f64, t3728: f64, t27464: f64, t3245: f64, t7928: f64, t12504: f64, t491: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t94638 = t7908 * t94585;
    let t94651 = t7895 * t27484;
    let t94656 = t1014 * t27332;
    let t94662 = t3728 * t27424;
    let t94664 = t1014 * t27464;
    let t94669 = t3245 * t7928;
    let t94743 = t12504 * t491;
    (t94638, t94651, t94656, t94662, t94664, t94669, t94743)
}
