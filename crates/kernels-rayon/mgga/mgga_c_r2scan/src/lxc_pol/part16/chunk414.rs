//! MGGA_C_R2SCAN lxc pol — lxc_pol part 16 (v4rho3sigma_6) CSE chunk 414/1264 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part16_v4rho3sigma_6_chunk414(t1691: f64, t721: f64, t695: f64, t713: f64, t194: f64, t685: f64, t63: f64, t1441: f64, t1442: f64, t1443: f64, t1444: f64, t1714: f64, t1717: f64) -> (f64, f64, f64, f64, f64) {
    let t1946 = t721 * t1691;
    let t1949 = t713 * t695;
    let t1956 = 1.0_f64 / t685 / t194;
    let t1957 = t63 * t1956;
    let t1966 = -0.39219166666666666667e0_f64 * t1714 + 0.31375333333333333333e1_f64 * t1717 + t1441 + t1442 + t1443 + t1444;
    (t1946, t1949, t1956, t1957, t1966)
}
