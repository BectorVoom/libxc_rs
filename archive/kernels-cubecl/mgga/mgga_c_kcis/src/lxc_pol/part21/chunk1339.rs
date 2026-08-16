//! MGGA_C_KCIS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 1339/1389 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part21_v4rho3sigma_3_chunk1339<F: Float>(t11223: F, t15095: F, t15098: F, t1872: F, t27100: F, t27141: F, t28073: F, t28253: F, t28260: F, t28295: F, t34650: F, t3664: F, t47711: F, t5394: F, t7812: F, t92410: F, t95503: F, t95506: F, t95508: F, t95510: F, t95514: F, t95517: F, t95520: F, t96542: F, t96545: F) -> F {
    let t96708 = F::cast_from(4.0_f64) * t11223 * t28073 + F::cast_from(4.0_f64) * t11223 * t28253 + F::cast_from(2.0_f64) * t15095 * t27141 + F::cast_from(4.0_f64) * t15098 * t27141 - t1872 * t92410 - F::cast_from(2.0_f64) * t27100 * t5394 - F::cast_from(12.0_f64) * t28260 * t34650 - F::cast_from(2.0_f64) * t28295 * t3664 + F::cast_from(4.0_f64) * t47711 * t7812 + t95503 + t95506 - t95508 - t95510 - t95514 - t95517 - t95520 - t96542 - t96545;
    t96708
}
