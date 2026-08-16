//! MGGA_C_REVTPSS lxc pol — lxc_pol part 28 (v4rho3sigma_3) CSE chunk 2225/2277 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2225(t25577: f64, t4817: f64, t15711: f64, t7132: f64, t15655: f64, t1972: f64, t16060: f64, t7111: f64, t25539: f64, t4924: f64, t1028: f64, t1656: f64, t1665: f64, t1675: f64, t25495: f64, t27479: f64, t3220: f64, t4854: f64, t4887: f64, t93592: f64, t93691: f64, t93715: f64, t93722: f64) -> f64 {
    let t100342 = 0.20325460441158986416e-2_f64 * t25577 * t4817;
    let t100343 = t7132 * t15711;
    let t100345 = t15655 * t1972;
    let t100359 = t7111 * t16060 / 432.0_f64;
    let t100363 = t25539 * t4924 / 162.0_f64;
    let t100364 = 0.96545937095505185476e-2_f64 * t93592 * t1675 - t100342 - 0.6351706387862183255e-4_f64 * t100343 - 0.85748036236139473944e-3_f64 * t100345 * t1028 - 0.42874018118069736972e-3_f64 * t27479 * t3220 + 0.45732285992607719436e-2_f64 * t93722 * t1665 + 0.45732285992607719436e-2_f64 * t25495 * t4854 - 0.14481890564325777821e-1_f64 * t93715 * t1665 - t25539 * t4887 / 54.0_f64 + t100359 + 11.0_f64 / 324.0_f64 * t93691 * t1656 - t100363;
    t100364
}
