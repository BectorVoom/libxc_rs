//! MGGA_C_RMGGAC lxc pol — lxc_pol part 37 (v4rho2sigma2_10) CSE chunk 1012/1128 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part37_v4rho2sigma2_10_chunk1012(t75820: f64, t75823: f64, t75825: f64, t2228: f64, t2350: f64, t903: f64, t15467: f64, t4601: f64, t1550: f64, t699: f64, t8704: f64, t75859: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t78311 = 0.5959043985061697516e-4_f64 * t75820;
    let t78312 = 0.2553875993597870364e-4_f64 * t75823;
    let t78313 = 0.2553875993597870364e-4_f64 * t75825;
    let t78321 = t903 * t2228 * t2350;
    let t78322 = 0.44903406381989282115e-1_f64 * t78321;
    let t78323 = t4601 * t15467;
    let t78324 = 0.44903406381989282115e-1_f64 * t78323;
    let t78326 = t1550 * t699 * t8704;
    let t78327 = 0.2993560425465952141e-1_f64 * t78326;
    let t78339 = 0.44903406381989282115e-1_f64 * t75859;
    (t78311, t78312, t78313, t78322, t78324, t78327, t78339)
}
