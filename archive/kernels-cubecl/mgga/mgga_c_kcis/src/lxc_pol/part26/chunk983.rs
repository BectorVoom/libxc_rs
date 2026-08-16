//! MGGA_C_KCIS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 983/1397 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part26_v4rho3sigma_8_chunk983<F: Float>(t22430: F, t6010: F, t1529: F, t7310: F, t1494: F, t21971: F, t572: F, t571: F, t22411: F, t22413: F, t22415: F, t22417: F, t22420: F, t22423: F, t22425: F, t22428: F) -> (F, F, F, F) {
    let t22431 = t6010 * t22430;
    let t22433 = t1529 * t7310;
    let t22435 = t1494 * t21971;
    let t22436 = t572 * t22435;
    let t22437 = t571 * t22436;
    let t22439 = -t22411 / F::cast_from(72.0_f64) + t22413 / F::cast_from(96.0_f64) - t22415 / F::cast_from(128.0_f64) - t22417 / F::cast_from(12.0_f64) + F::cast_from(11.0_f64) / F::cast_from(27.0_f64) * t22420 - F::cast_from(19.0_f64) / F::cast_from(108.0_f64) * t22423 + t22425 / F::cast_from(128.0_f64) + F::cast_from(19.0_f64) / F::cast_from(144.0_f64) * t22428 - t22431 / F::cast_from(64.0_f64) - t22433 / F::cast_from(72.0_f64) + t22437 / F::cast_from(24.0_f64);
    (t22431, t22433, t22437, t22439)
}
