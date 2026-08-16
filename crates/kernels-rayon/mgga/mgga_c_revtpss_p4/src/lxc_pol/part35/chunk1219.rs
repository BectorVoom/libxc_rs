//! MGGA_C_REVTPSS lxc pol — lxc_pol part 35 (v4rho3sigma_10) CSE chunk 1219/1234 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part35_v4rho3sigma_10_chunk1219(t103404: f64, t103422: f64, t103432: f64, t110525: f64, t110527: f64, t110542: f64, t110544: f64, t110572: f64, t110584: f64, t110591: f64, t23384: f64, t23404: f64, t25317: f64, t28394: f64, t29698: f64, t6048: f64, t6072: f64, t7070: f64, t7403: f64, t7997: f64, t8016: f64) -> f64 {
    let t115637 = -0.43368140941025997312e-1_f64 * t110525 + 0.77108554593144223218e-1_f64 * t110527 - 0.51405703062096148812e-1_f64 * t103404 - 0.78062653693846795158e1_f64 * t7070 * t25317 * t7997 * t6048 + 0.39512695097613069591e1_f64 * t7403 * t23404 + 0.51405703062096148812e-1_f64 * t103422 - 0.21684070470512998656e-1_f64 * t110542 + 0.38554277296572111609e-1_f64 * t110544 - 0.10281140612419229762e0_f64 * t103432 + 0.13010442282307799194e0_f64 * t110572 - 0.32927245914677557992e-1_f64 * t110584 - 0.19756347548806534796e1_f64 * t28394 * t6072 + 0.32927245914677557992e-1_f64 * t110591 - 0.13010442282307799193e1_f64 * t29698 * t8016 - 0.65854491829355115987e0_f64 * t7403 * t23384;
    t115637
}
