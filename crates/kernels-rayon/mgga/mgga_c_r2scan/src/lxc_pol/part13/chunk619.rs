//! MGGA_C_R2SCAN lxc pol — lxc_pol part 13 (v4rho3sigma_3) CSE chunk 619/1253 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part13_v4rho3sigma_3_chunk619(t3386: f64, t341: f64, t1083: f64, t1085: f64, t1087: f64, t1089: f64, t1091: f64, t3388: f64, t3390: f64, t3394: f64, t3398: f64, t3402: f64, t343: f64, t839: f64) -> (f64, f64) {
    let t3406 = t341 * t3386;
    let t3412 = -0.64e0_f64 * t3386 - 0.8704e0_f64 * t3388 - 0.8704e0_f64 * t3390 - 0.9214113627294e1_f64 * t1083 * t839 - 0.4607056813647e1_f64 * t3394 + 0.367387230261e2_f64 * t1085 * t839 + 0.122462410087e2_f64 * t3398 - 0.3831420472412e2_f64 * t1087 * t839 - 0.957855118103e1_f64 * t3402 + 0.1550653405116e2_f64 * t1089 * t839 + 0.3101306810232e1_f64 * t3406 - 0.2177652951264e1_f64 * t1091 * t839 - 0.362942158544e0_f64 * t343 * t3386;
    (t3406, t3412)
}
