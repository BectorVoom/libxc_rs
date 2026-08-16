//! MGGA_C_R2SCAN lxc pol — lxc_pol part 15 (v4rho3sigma_5) CSE chunk 1244/1253 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part15_v4rho3sigma_5_chunk1244(t1020: f64, t1081: f64, t1087: f64, t11110: f64, t11115: f64, t11930: f64, t1310: f64, t1312: f64, t2410: f64, t333: f64, t335: f64, t337: f64, t3386: f64, t339: f64, t3398: f64, t341: f64, t343: f64, t3648: f64, t3656: f64, t40893: f64, t839: f64, t8438: f64) -> f64 {
    let t41019 = 0.367387230261e2_f64 * t3656 * t1310 - 0.3831420472412e2_f64 * t11115 * t1020 - 0.7662840944824e2_f64 * t3398 * t2410 - 0.3831420472412e2_f64 * t1087 * t8438 - 0.362942158544e0_f64 * t343 * t40893 - 0.8704e0_f64 * t8438 * t1081 - 0.17408e1_f64 * t2410 * t3386 - 0.8704e0_f64 * t1020 * t11110 - 0.8704e0_f64 * t1310 * t3648 - 0.17408e1_f64 * t839 * t11930 - 0.8704e0_f64 * t333 * t40893 - 0.4607056813647e1_f64 * t335 * t40893 + 0.122462410087e2_f64 * t337 * t40893 - 0.957855118103e1_f64 * t339 * t40893 + 0.3101306810232e1_f64 * t341 * t40893 - 0.9214113627294e1_f64 * t1312 * t3648;
    t41019
}
