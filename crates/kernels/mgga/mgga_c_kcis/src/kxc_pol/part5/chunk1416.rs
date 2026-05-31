//! MGGA_C_KCIS kxc pol — kxc_pol part 5 (v3rho3_2) CSE chunk 1416/1419 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_kxc_pol_part5_v3rho3_2_chunk1416<F: Float>(t23379: F, t449: F, t446: F, t2132: F, t5407: F, t6290: F, t908: F, t1881: F, t5414: F, t13000: F, t13096: F, t18385: F, t20856: F, t20859: F, t20861: F, t20863: F, t20866: F, t20870: F, t9267: F, t9270: F, t9278: F, t9281: F) -> F {
    let t23380 = t449 * t23379;
    let t23381 = t446 * t23380;
    let t23383 = t5407 * t2132;
    let t23384 = t446 * t23383;
    let t23386 = t6290 * t908;
    let t23387 = t1881 * t5414;
    let t23389 = -t20856 / F::cast_from(8.0_f64) - t20859 / F::cast_from(16.0_f64) + t13096 + t20861 / F::cast_from(8.0_f64) + t20863 / F::cast_from(16.0_f64) - t20866 / F::cast_from(16.0_f64) + F::cast_from(2.0_f64) * t18385 - t20870 / F::cast_from(16.0_f64) - t9278 + t9267 - t23381 / F::cast_from(16.0_f64) + t9281 - t23384 / F::cast_from(8.0_f64) + t23386 + t23387 / F::cast_from(8.0_f64) - t9270 + t13000;
    t23389
}
