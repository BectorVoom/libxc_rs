//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 54 (v4rho2sigma2_10) CSE chunk 1395/1484 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part54_v4rho2sigma2_10_chunk1395(t1880: f64, t25216: f64, t31366: f64, t121401: f64, t6572: f64, t114944: f64, t114945: f64, t118913: f64, t118916: f64, t118917: f64, t118918: f64, t13053: f64, t13065: f64, t1912: f64, t26713: f64, t31400: f64, t31423: f64, t4147: f64, t4301: f64, t6663: f64, t8563: f64, t92439: f64) -> f64 {
    let t121713 = t1880 * t31366 * t25216;
    let t121716 = t1880 * t121401 * t6572;
    let t121725 = -t118913 - 0.82246703342411321825e-2_f64 * t121713 + t118916 - 0.82246703342411321825e-2_f64 * t121716 - t4147 * t31400 - t26713 * t6663 + t114944 - t31423 * t4301 - t92439 * t1912 + t118917 + t118918 - t13053 * t8563 - t13065 * t8563 + 0.19190897446562641759e-1_f64 * t114945;
    t121725
}
