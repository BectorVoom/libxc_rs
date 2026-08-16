//! MGGA_C_REVTPSS lxc pol — lxc_pol part 25 (v4rho3sigma_0) CSE chunk 1057/1360 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part25_v4rho3sigma_0_chunk1057(t1211: f64, t12621: f64, t1207: f64, t456: f64, t487: f64, t1214: f64, t3568: f64, t1269: f64, t3566: f64, t1203: f64, t3565: f64, t3584: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t12622 = t1211 * t12621;
    let t12625 = t1207 * t1207;
    let t12626 = 1.0_f64 / t12625;
    let t12627 = t456 * t12626;
    let t12628 = t12627 * t487;
    let t12629 = t3568 * t1214;
    let t12630 = t1211 * t12629;
    let t12633 = t3566 * t1269;
    let t12640 = t1203 * t3565;
    let t12641 = t12640 * t487;
    let t12646 = t1214 * t3584;
    (t12622, t12627, t12628, t12629, t12630, t12633, t12640, t12641, t12646)
}
