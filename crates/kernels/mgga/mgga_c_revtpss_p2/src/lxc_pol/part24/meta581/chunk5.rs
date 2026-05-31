//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1810/1850 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1810<F: Float>(t30: F, t33: F, t6785: F, t5824: F, t1344: F, t21944: F, t22670: F, t3874: F, t46310: F, t5574: F, t87125: F, t6792: F, t6416: F, t1348: F, t21956: F, t22783: F, t3881: F, t46328: F, t5582: F, t89780: F, zeta_threshold: F) -> (F, F, F, F, F, F) {
    let t31 = t30 <= zeta_threshold;
    let t34 = t33 <= zeta_threshold;
    let t91797 = t6785 * t6785;
    let t91802 = t5824 * t5824;
    let t91810 = piecewise3::<F>(t31, F::cast_from(0.0_f64), -F::cast_from(56.0_f64) / F::cast_from(81.0_f64) * t46310 * t91797 + F::cast_from(16.0_f64) / F::cast_from(9.0_f64) * t21944 * t5824 - F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t3874 * t91802 - F::cast_from(8.0_f64) / F::cast_from(9.0_f64) * t5574 * t22670 + F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t1344 * t87125);
    let t91811 = t6792 * t6792;
    let t91816 = t6416 * t6416;
    let t91824 = piecewise3::<F>(t34, F::cast_from(0.0_f64), -F::cast_from(56.0_f64) / F::cast_from(81.0_f64) * t46328 * t91811 + F::cast_from(16.0_f64) / F::cast_from(9.0_f64) * t21956 * t6416 - F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t3881 * t91816 - F::cast_from(8.0_f64) / F::cast_from(9.0_f64) * t5582 * t22783 + F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t1348 * t89780);
    (t91797, t91802, t91810, t91811, t91816, t91824)
}
