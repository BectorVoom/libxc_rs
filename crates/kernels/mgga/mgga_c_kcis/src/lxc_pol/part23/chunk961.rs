//! MGGA_C_KCIS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 961/1323 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part23_v4rho3sigma_5_chunk961<F: Float>(t16609: F, t584: F, t583: F, t1546: F, t17484: F, t17488: F, t17491: F, t17494: F, t17497: F, t17499: F, t17502: F, t17506: F, t17510: F, t17512: F, t17515: F, t17518: F, t17521: F, t17693: F, t17695: F, t17698: F, t17700: F) -> (F, F, F) {
    let t17702 = t584 * t16609;
    let t17703 = t583 * t17702;
    let t17704 = t1546 * t17703;
    let t17706 = t17484 / F::new(256.0) - t17488 / F::new(16.0) + t17491 / F::new(12.0) - t17494 / F::new(9.0) + t17497 / F::new(6.0) + t17499 / F::new(18.0) - t17502 / F::new(48.0) - t17506 / F::new(18.0) - F::new(3.0) / F::new(8.0) * t17510 - t17512 / F::new(192.0) + t17515 / F::new(128.0) - t17518 / F::new(128.0) + t17521 / F::new(27.0) + t17693 / F::new(16.0) + t17695 / F::new(256.0) + t17698 / F::new(36.0) - t17700 / F::new(6.0) + t17704 / F::new(256.0);
    (t17703, t17704, t17706)
}
