//! MGGA_C_R2SCAN lxc pol — lxc_pol part 18 (v4rho3sigma_8) CSE chunk 647/1264 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part18_v4rho3sigma_8_chunk647<F: Float>(t341: F, t3648: F, t1020: F, t1083: F, t1085: F, t1087: F, t1089: F, t1091: F, t343: F, t3650: F, t3652: F, t3656: F, t3660: F, t3664: F) -> (F, F) {
    let t3668 = t341 * t3648;
    let t3674 = -F::new(0.64e0) * t3648 - F::new(0.8704e0) * t3650 - F::new(0.8704e0) * t3652 - F::cast_from(0.9214113627294e1_f64) * t1083 * t1020 - F::cast_from(0.4607056813647e1_f64) * t3656 + F::cast_from(0.367387230261e2_f64) * t1085 * t1020 + F::cast_from(0.122462410087e2_f64) * t3660 - F::cast_from(0.3831420472412e2_f64) * t1087 * t1020 - F::cast_from(0.957855118103e1_f64) * t3664 + F::cast_from(0.1550653405116e2_f64) * t1089 * t1020 + F::cast_from(0.3101306810232e1_f64) * t3668 - F::cast_from(0.2177652951264e1_f64) * t1091 * t1020 - F::cast_from(0.362942158544e0_f64) * t343 * t3648;
    (t3668, t3674)
}
