//! MGGA_C_REVTPSS lxc pol — lxc_pol part 25 (v4rho3sigma_0) CSE chunk 1317/1360 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part25_v4rho3sigma_0_chunk1317(t2018: f64, t807: f64, t9703: f64, t3994: f64, t7028: f64, t9845: f64, t25240: f64, t3951: f64, t3964: f64, t25972: f64, t9761: f64, t2681: f64, t7269: f64, t820: f64) -> (f64, f64, f64, f64, f64) {
    let t94534 = t807 * t2018 * t9703;
    let t94537 = t9845 * t7028 * t3994;
    let t94540 = t3964 * t25240 * t3951;
    let t94542 = t25972 * t9761;
    let t94545 = t820 * t7269 * t2681;
    (t94534, t94537, t94540, t94542, t94545)
}
