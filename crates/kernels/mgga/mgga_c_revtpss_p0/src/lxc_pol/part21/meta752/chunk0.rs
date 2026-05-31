//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2630/3259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2630<F: Float>(t30: F, t1868: F, t9940: F, t5577: F, t588: F, t1344: F, t13687: F, t13690: F, t1468: F, t2: F, t22: F, t3874: F, t46310: F, t48165: F, t48168: F, t48174: F, t48177: F, t5574: F, t580: F, t605: F, t9336: F, t9344: F, t9605: F, zeta_threshold: F) -> (F, F) {
    let t31 = t30 <= zeta_threshold;
    let t48347 = t9940 * t1868;
    let t48394 = F::cast_from(16.0_f64) * t5577 * t588;
    let t48396 = piecewise3::<F>(t31, F::cast_from(0.0_f64), -F::cast_from(56.0_f64) / F::cast_from(81.0_f64) * t46310 * t1468 * t9336 + F::cast_from(16.0_f64) / F::cast_from(9.0_f64) * t9605 * t2 * t48165 + F::cast_from(8.0_f64) / F::cast_from(9.0_f64) * t13687 * t48168 - F::cast_from(4.0_f64) / F::cast_from(3.0_f64) * t3874 * t580 * t605 + F::cast_from(4.0_f64) * t13690 * t48174 - F::cast_from(4.0_f64) / F::cast_from(3.0_f64) * t13690 * t48177 - F::cast_from(2.0_f64) / F::cast_from(9.0_f64) * t5574 * t9344 - F::cast_from(8.0_f64) * t1344 * t22 + t48394);
    (t48347, t48396)
}
