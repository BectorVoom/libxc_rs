//! MGGA_C_R2SCAN lxc pol — lxc_pol part 18 (v4rho3sigma_8) CSE chunk 1152/1264 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part18_v4rho3sigma_8_chunk1152(t1020: f64, t1081: f64, t1085: f64, t1087: f64, t1089: f64, t11930: f64, t12659: f64, t2410: f64, t2956: f64, t2958: f64, t333: f64, t335: f64, t337: f64, t3386: f64, t339: f64, t341: f64, t343: f64, t3648: f64, t42616: f64, t839: f64, t9707: f64, t9709: f64, t9715: f64) -> f64 {
    let t42742 = 0.18607840861392e3_f64 * t1085 * t9715 + 0.12405227240928e3_f64 * t1087 * t9709 - 0.4355305902528e2_f64 * t1087 * t9715 - 0.2177652951264e2_f64 * t1089 * t9709 + 0.122462410087e2_f64 * t337 * t42616 - 0.957855118103e1_f64 * t339 * t42616 + 0.3101306810232e1_f64 * t341 * t42616 - 0.362942158544e0_f64 * t343 * t42616 - 0.8704e0_f64 * t839 * t12659 - 0.8704e0_f64 * t333 * t42616 - 0.4607056813647e1_f64 * t335 * t42616 - 0.9214113627294e1_f64 * t2958 * t3386 - 0.8704e0_f64 * t9707 * t1081 - 0.8704e0_f64 * t2956 * t3386 - 0.17408e1_f64 * t2410 * t3648 - 0.17408e1_f64 * t1020 * t11930;
    t42742
}
