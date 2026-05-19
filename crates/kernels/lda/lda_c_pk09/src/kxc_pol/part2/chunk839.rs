//! LDA_C_PK09 kxc pol — kxc_pol part 2 (v2rho2_1) CSE chunk 839/1113 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pk09_kxc_pol_part2_v2rho2_1_chunk839<F: Float>(t192: F, t2214: F, t2314: F, t3753: F, t4411: F, t4660: F, t709: F, t713: F, t7706: F, t7727: F, t7776: F, t8555: F, t8560: F, t8564: F, t8566: F, t8571: F, t8573: F) -> F {
    let t8575 = F::cast_from(2.427516195194328_f64) * t3753 * t2214 + F::cast_from(2.2140749178833072_f64) * t192 * t7776 + F::cast_from(2.2140749178833072_f64) * t192 * t7706 - F::cast_from(1.8805371096875316_f64) * t8555 * t713 - F::cast_from(1.8805371096875316_f64) * t8555 * t709 + F::cast_from(19.489173774580152_f64) * t8560 + F::cast_from(2.2140749178833072_f64) * t7727 * t713 + F::cast_from(12.992782516386768_f64) * t8564 + F::cast_from(1.2536914064583544_f64) * t8566 - t4660 * t2314 + F::cast_from(14.71989892086604_f64) * t4411 + F::cast_from(2.2140749178833072_f64) * t8571 - F::cast_from(3.2915558116322368_f64) * t8573;
    t8575
}
