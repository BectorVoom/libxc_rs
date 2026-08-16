//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1017/1850 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1017(t15670: f64, t366: f64, t245: f64, t4890: f64, t3088: f64, t3317: f64, t372: f64, t4823: f64, t1087: f64, t11773: f64, t4801: f64, t1062: f64, t4857: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t15671 = t15670 * t366;
    let t15687 = t4890 * t245;
    let t15688 = t3088 * t15687;
    let t15689 = t3317 * t15688;
    let t15696 = t372 * t4823;
    let t15700 = t1087 * t11773;
    let t15701 = t372 * t4801;
    let t15707 = t4857 * t1062;
    (t15671, t15687, t15688, t15689, t15696, t15700, t15701, t15707)
}
