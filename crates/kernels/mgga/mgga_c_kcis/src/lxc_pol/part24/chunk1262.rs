//! MGGA_C_KCIS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 1262/1322 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part24_v4rho3sigma_6_chunk1262<F: Float>(t1014: F, t28966: F, t19811: F, t7718: F, t9370: F, t100636: F, t100641: F, t28190: F, t28211: F, t8087: F, t93016: F, t96068: F, t97212: F, t97248: F, t97250: F, t97267: F) -> (F, F, F) {
    let t100643 = t1014 * t28966;
    let t100646 = t9370 * t7718 * t19811;
    let t100652 = F::cast_from(0.20594135802469135803e-3_f64) * t97212 + F::cast_from(0.46429444444444444443e-2_f64) * t100636 - F::cast_from(0.41270617283950617283e-2_f64) * t96068 - F::cast_from(0.20594135802469135803e-3_f64) * t93016 - F::cast_from(0.61905925925925925925e-2_f64) * t100641 + F::cast_from(0.15476481481481481481e-2_f64) * t100643 + F::cast_from(0.51588271604938271605e-2_f64) * t100646 - t97248 - t97250 + F::cast_from(0.69505208333333333334e-3_f64) * t28190 * t28211 + F::cast_from(0.69505208333333333334e-3_f64) * t97267 * t8087;
    (t100643, t100646, t100652)
}
