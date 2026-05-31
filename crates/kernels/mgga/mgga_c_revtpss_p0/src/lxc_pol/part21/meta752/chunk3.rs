//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2633/3259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2633<F: Float>(t13768: F, t13902: F, t13907: F, t1877: F, t22229: F, t225: F, t4045: F, t4053: F, t48220: F, t48245: F, t48257: F, t48272: F, t48289: F, t48309: F, t48321: F, t48337: F, t48347: F, t48436: F, t541: F, t543: F, t5644: F, t5650: F, t5652: F, t5655: F, t73: F, t9400: F, t9881: F, t9884: F, t9887: F, t9984: F) -> F {
    let t48438 = (F::cast_from(9.0_f64) * t5644 * t4053 + F::cast_from(180.0_f64) * t13902 * t13907 - F::cast_from(36.0_f64) * t4045 * t73 * t5652 - (t48220 + t48245 + t48257 + t48272 + t48289 + t48309 + t48321 + t48337) * t225 * t541 + F::cast_from(60.0_f64) * t1877 * t9881 - F::cast_from(36.0_f64) * t22229 * t9884 - F::cast_from(360.0_f64) * t5650 * t48347 * t9400 + F::cast_from(180.0_f64) * t5650 * t13768 * t9984 + F::cast_from(9.0_f64) * t4045 * t5655 + F::cast_from(3.0_f64) * t1877 * t9887 + t48436) * t543;
    t48438
}
