//! MGGA_C_R2SCAN lxc pol — lxc_pol part 15 (v4rho3sigma_5) CSE chunk 326/1253 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part15_v4rho3sigma_5_chunk326<F: Float>(t322: F, t1080: F, t1081: F, t1083: F, t1085: F, t1087: F, t1089: F, t1091: F, t1096: F, t343: F, t352: F, t855: F, t259: F, t869: F) -> (F, F) {
    let t323 = t322 <= F::new(0.0);
    let t331 = t322 <= F::new(0.25e1);
    let t1100 = piecewise5::<f64>(t323, t1080, t331, -F::new(0.64e0) * t1081 - F::new(0.8704e0) * t1083 - F::new(0.4607056813647e1) * t1085 + F::new(0.122462410087e2) * t1087 - F::new(0.957855118103e1) * t1089 + F::new(0.3101306810232e1) * t1091 - F::new(0.362942158544e0) * t343 * t1081, -F::new(0.105e1) * t855 * t1096 * t352);
    let t1102 = t869 * t259;
    (t1100, t1102)
}
