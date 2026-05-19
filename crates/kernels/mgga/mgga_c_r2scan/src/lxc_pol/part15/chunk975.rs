//! MGGA_C_R2SCAN lxc pol — lxc_pol part 15 (v4rho3sigma_5) CSE chunk 975/1253 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part15_v4rho3sigma_5_chunk975<F: Float>(t11110: F, t339: F, t341: F, t1083: F, t1085: F, t1087: F, t1089: F, t1091: F, t1310: F, t1312: F, t3402: F, t3406: F, t343: F, t839: F) -> (F, F, F) {
    let t11118 = t339 * t11110;
    let t11120 = t341 * t11110;
    let t11141 = -F::cast_from(0.957855118103e1_f64) * t11118 + F::cast_from(0.3101306810232e1_f64) * t11120 - F::cast_from(0.362942158544e0_f64) * t343 * t11110 + F::cast_from(0.734774460522e2_f64) * t1083 * t1312 - F::cast_from(0.11494261417236e3_f64) * t1085 * t1312 + F::cast_from(0.6202613620464e2_f64) * t1087 * t1312 - F::cast_from(0.1088826475632e2_f64) * t1089 * t1312 - F::new(0.64e0) * t11110 + F::cast_from(0.3101306810232e2_f64) * t3402 * t839 + F::cast_from(0.1550653405116e2_f64) * t1089 * t1310 - F::cast_from(0.4355305902528e1_f64) * t3406 * t839 - F::cast_from(0.2177652951264e1_f64) * t1091 * t1310;
    (t11118, t11120, t11141)
}
