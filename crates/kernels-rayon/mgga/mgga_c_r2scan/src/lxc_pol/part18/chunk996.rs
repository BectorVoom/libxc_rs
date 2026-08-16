//! MGGA_C_R2SCAN lxc pol — lxc_pol part 18 (v4rho3sigma_8) CSE chunk 996/1264 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part18_v4rho3sigma_8_chunk996(t1083: f64, t1085: f64, t1087: f64, t1089: f64, t1091: f64, t11930: f64, t11975: f64, t11977: f64, t11979: f64, t11981: f64, t11983: f64, t11985: f64, t11987: f64, t11989: f64, t2410: f64, t2412: f64, t343: f64, t3668: f64, t839: f64) -> f64 {
    let t11991 = -0.2177652951264e1_f64 * t1091 * t2410 - 0.2177652951264e1_f64 * t3668 * t839 + 0.734774460522e2_f64 * t1083 * t2412 - 0.11494261417236e3_f64 * t1085 * t2412 + 0.6202613620464e2_f64 * t1087 * t2412 - 0.1088826475632e2_f64 * t1089 * t2412 - 0.362942158544e0_f64 * t343 * t11930 - 0.8704e0_f64 * t11975 - 0.8704e0_f64 * t11977 - 0.8704e0_f64 * t11979 - 0.8704e0_f64 * t11981 - 0.4607056813647e1_f64 * t11983 + 0.122462410087e2_f64 * t11985 - 0.957855118103e1_f64 * t11987 + 0.3101306810232e1_f64 * t11989;
    t11991
}
