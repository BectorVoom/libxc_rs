//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 55 (v4rho2sigma2_11) CSE chunk 1292/1304 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part55_v4rho2sigma2_11_chunk1292(t12571: f64, t32582: f64, t79: f64, t7973: f64, t117710: f64, t117734: f64, t117737: f64, t119892: f64, t119902: f64, t119909: f64, t119917: f64, t119924: f64, t119928: f64, t119932: f64, t119933: f64, t119948: f64, t31: f64, t31013: f64, t31860: f64, t31864: f64, t32579: f64, t32583: f64, t33106: f64, t33111: f64, t33118: f64, t34221: f64, t607: f64, t641: f64, t645: f64, t7254: f64, t8308: f64, t8513: f64, t8663: f64, t8855: f64) -> f64 {
    let t125865 = t12571 * t32582;
    let t125889 = t79 * t7973;
    let t125900 = -5.0_f64 / 18.0_f64 * t117710 * t119892 - 5.0_f64 / 18.0_f64 * t31864 * t8308 * t7973 * t31 * t607 - 5.0_f64 / 18.0_f64 * t117710 * t119902 - 5.0_f64 / 36.0_f64 * t125865 * t31013 + 35.0_f64 / 24.0_f64 * t117737 * t119909 - 5.0_f64 / 12.0_f64 * t31860 * t8513 * t33106 * t7254 - 5.0_f64 / 12.0_f64 * t32579 * t119917 - 5.0_f64 / 36.0_f64 * t117734 * t33111 - 5.0_f64 / 36.0_f64 * t32583 * t119924 - 5.0_f64 / 36.0_f64 * t32583 * t119928 + 5.0_f64 / 18.0_f64 * t119932 * t8855 * t119933 - 5.0_f64 / 12.0_f64 * t31860 * t8513 * t34221 * t645 + 5.0_f64 / 36.0_f64 * t8663 * t8513 * t125889 * t641 - 5.0_f64 / 12.0_f64 * t32579 * t119948 + 5.0_f64 / 36.0_f64 * t8663 * t8513 * t33118 * t7254;
    t125900
}
