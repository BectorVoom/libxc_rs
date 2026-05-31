//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 3255/3259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3255<F: Float>(t117: F, t60498: F, t10259: F, t93: F, t10416: F, t1312: F, t13426: F, t13435: F, t13440: F, t13514: F, t1518: F, t18227: F, t2322: F, t2371: F, t4248: F, t4292: F, t46126: F, t49686: F, t49693: F, t49830: F, t49851: F, t5523: F, t60206: F, t670: F) -> (F, F) {
    let t60499 = t60498 * t117;
    let t60551 = t93 * t10259;
    let t60556 = F::cast_from(2.0_f64) * t10259 * t4248 + F::cast_from(6.0_f64) * t10416 * t4292 + F::cast_from(2.0_f64) * t1312 * t49830 + F::cast_from(6.0_f64) * t13426 * t2371 + F::cast_from(12.0_f64) * t13435 * t4292 + F::cast_from(6.0_f64) * t13440 * t4292 + F::cast_from(6.0_f64) * t13514 * t2322 + F::cast_from(6.0_f64) * t13514 * t5523 + F::cast_from(2.0_f64) * t1518 * t46126 + F::cast_from(6.0_f64) * t1518 * t49693 + F::cast_from(6.0_f64) * t1518 * t49851 + F::cast_from(2.0_f64) * t1518 * t60551 + F::cast_from(6.0_f64) * t18227 * t2371 + F::cast_from(6.0_f64) * t49686 * t670 + F::cast_from(6.0_f64) * t60206 + t60499;
    (t60499, t60556)
}
