//! MGGA_C_KCIS lxc pol — lxc_pol part 4 (v3rho3_1) CSE chunk 1372/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part4_v3rho3_1_chunk1372<F: Float>(t16609: F, t584: F, t583: F, t1546: F, t17484: F, t17488: F, t17491: F, t17494: F, t17497: F, t17499: F, t17502: F, t17506: F, t17510: F, t17512: F, t17515: F, t17518: F, t17521: F, t17693: F, t17695: F, t17698: F, t17700: F) -> (F, F) {
    let t17702 = t584 * t16609;
    let t17703 = t583 * t17702;
    let t17704 = t1546 * t17703;
    let t17706 = t17484 / F::cast_from(256.0_f64) - t17488 / F::cast_from(16.0_f64) + t17491 / F::cast_from(12.0_f64) - t17494 / F::cast_from(9.0_f64) + t17497 / F::cast_from(6.0_f64) + t17499 / F::cast_from(18.0_f64) - t17502 / F::cast_from(48.0_f64) - t17506 / F::cast_from(18.0_f64) - F::cast_from(3.0_f64) / F::cast_from(8.0_f64) * t17510 - t17512 / F::cast_from(192.0_f64) + t17515 / F::cast_from(128.0_f64) - t17518 / F::cast_from(128.0_f64) + t17521 / F::cast_from(27.0_f64) + t17693 / F::cast_from(16.0_f64) + t17695 / F::cast_from(256.0_f64) + t17698 / F::cast_from(36.0_f64) - t17700 / F::cast_from(6.0_f64) + t17704 / F::cast_from(256.0_f64);
    (t17704, t17706)
}
