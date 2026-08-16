//! MGGA_C_REVTPSS lxc pol — lxc_pol part 34 (v4rho3sigma_9) CSE chunk 1237/1341 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part34_v4rho3sigma_9_chunk1237(t15711: f64, t7132: f64, t16219: f64, t7111: f64, t11239: f64, t1678: f64, t1078: f64, t1982: f64, t3143: f64, t7810: f64, t3057: f64, t25698: f64, t27418: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t100343 = t7132 * t15711;
    let t100365 = t7111 * t16219;
    let t100533 = t1678 * t11239;
    let t100535 = t1982 * t100533 * t1078;
    let t100567 = t3143 * t7810;
    let t100681 = t3057 * t7810;
    let t100705 = t25698 * t27418;
    (t100343, t100365, t100535, t100567, t100681, t100705)
}
