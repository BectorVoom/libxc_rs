//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 1323/1395 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk1323(t10514: f64, t64975: f64, t18246: f64, t35525: f64, t1398: f64, t2829: f64, t19809: f64, t61703: f64, t44329: f64, t1364: f64, t10662: f64, t20011: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t64976 = t64975 * t10514;
    let t64979 = t18246 * t35525;
    let t64982 = t2829 * t1398;
    let t64986 = t61703 * t19809;
    let t64989 = t18246 * t44329;
    let t64992 = t2829 * t1364;
    let t64997 = t20011 * t10662;
    (t64976, t64979, t64982, t64986, t64989, t64992, t64997)
}
