//! MGGA_C_R2SCAN lxc pol — lxc_pol part 13 (v4rho3sigma_3) CSE chunk 1018/1253 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part13_v4rho3sigma_3_chunk1018<F: Float>(t1083: F, t1085: F, t1087: F, t1089: F, t1091: F, t11930: F, t11975: F, t11977: F, t11979: F, t11981: F, t11983: F, t11985: F, t11987: F, t11989: F, t2410: F, t2412: F, t343: F, t3668: F, t839: F) -> F {
    let t11991 = -F::cast_from(0.2177652951264e1_f64) * t1091 * t2410 - F::cast_from(0.2177652951264e1_f64) * t3668 * t839 + F::cast_from(0.734774460522e2_f64) * t1083 * t2412 - F::cast_from(0.11494261417236e3_f64) * t1085 * t2412 + F::cast_from(0.6202613620464e2_f64) * t1087 * t2412 - F::cast_from(0.1088826475632e2_f64) * t1089 * t2412 - F::cast_from(0.362942158544e0_f64) * t343 * t11930 - F::cast_from(0.8704e0_f64) * t11975 - F::cast_from(0.8704e0_f64) * t11977 - F::cast_from(0.8704e0_f64) * t11979 - F::cast_from(0.8704e0_f64) * t11981 - F::cast_from(0.4607056813647e1_f64) * t11983 + F::cast_from(0.122462410087e2_f64) * t11985 - F::cast_from(0.957855118103e1_f64) * t11987 + F::cast_from(0.3101306810232e1_f64) * t11989;
    t11991
}
