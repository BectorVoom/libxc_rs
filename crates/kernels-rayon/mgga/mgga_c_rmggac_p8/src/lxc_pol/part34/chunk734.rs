//! MGGA_C_RMGGAC lxc pol — lxc_pol part 34 (v4rho2sigma2_7) CSE chunk 734/1097 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part34_v4rho2sigma2_7_chunk734(t70904: f64, t14683: f64, t7244: f64, t14551: f64, t7508: f64, t68735: f64, t235: f64, t29837: f64, t698: f64, t2046: f64, t2050: f64, t2232: f64, t31: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t70905 = 0.43368970657079495312e-4_f64 * t70904;
    let t70929 = t7244 * t14683;
    let t70948 = t7508 * t14551;
    let t71005 = 0.54934029498967360725e-3_f64 * t68735;
    let t71007 = t235 * t29837 * t698;
    let t71021 = t2046 * t2050 * t2232 * t31;
    (t70905, t70929, t70948, t71005, t71007, t71021)
}
