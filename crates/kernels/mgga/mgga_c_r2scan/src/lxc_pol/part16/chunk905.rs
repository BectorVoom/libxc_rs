//! MGGA_C_R2SCAN lxc pol — lxc_pol part 16 (v4rho3sigma_6) CSE chunk 905/1112 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part16_v4rho3sigma_6_chunk905<F: Float>(t1081: F, t2410: F, t1020: F, t3386: F, t3648: F, t839: F, t11930: F, t333: F, t335: F, t337: F, t339: F, t341: F, t1083: F, t1085: F, t1087: F, t1089: F, t1091: F, t2412: F, t343: F, t3668: F) -> (F, F, F, F, F, F, F) {
    let t11975 = t2410 * t1081;
    let t11977 = t1020 * t3386;
    let t11979 = t839 * t3648;
    let t11981 = t333 * t11930;
    let t11983 = t335 * t11930;
    let t11985 = t337 * t11930;
    let t11987 = t339 * t11930;
    let t11989 = t341 * t11930;
    let t11991 = -0.2177652951264e1 * t1091 * t2410 - 0.2177652951264e1 * t3668 * t839 + 0.734774460522e2 * t1083 * t2412 - 0.11494261417236e3 * t1085 * t2412 + 0.6202613620464e2 * t1087 * t2412 - 0.1088826475632e2 * t1089 * t2412 - 0.362942158544e0 * t343 * t11930 - 0.8704e0 * t11975 - 0.8704e0 * t11977 - 0.8704e0 * t11979 - 0.8704e0 * t11981 - 0.4607056813647e1 * t11983 + 0.122462410087e2 * t11985 - 0.957855118103e1 * t11987 + 0.3101306810232e1 * t11989;
    (t11979, t11981, t11983, t11985, t11987, t11989, t11991)
}
