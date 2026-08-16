//! MGGA_C_KCIS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 781/1323 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part23_v4rho3sigma_5_chunk781(t11814: f64, t4155: f64, t1465: f64, t540: f64, t3728: f64, t3956: f64, t4131: f64, t1392: f64, t1457: f64, t1017: f64, t86: f64, sigma2: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t11815 = t11814 * t4155;
    let t11823 = t1465 * t1465;
    let t11824 = 1.0_f64 / t11823;
    let t11825 = t540 * t11824;
    let t11826 = t11825 * sigma2;
    let t11832 = t3728 * t3956;
    let t11838 = t3728 * t4131;
    let t11860 = t1392 * t1457;
    let t11862 = t86 * t1017 * t11860;
    (t11815, t11824, t11825, t11826, t11832, t11838, t11862)
}
