//! MGGA_C_KCIS lxc pol — lxc_pol part 5 (v3rho3_2) CSE chunk 1363/1419 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part5_v3rho3_2_chunk1363<F: Float>(t22430: F, t6010: F, t1529: F, t7310: F, t1494: F, t21971: F, t572: F, t571: F, t22411: F, t22413: F, t22415: F, t22417: F, t22420: F, t22423: F, t22425: F, t22428: F) -> (F, F, F, F) {
    let t22431 = t6010 * t22430;
    let t22433 = t1529 * t7310;
    let t22435 = t1494 * t21971;
    let t22436 = t572 * t22435;
    let t22437 = t571 * t22436;
    let t22439 = -t22411 / F::new(72.0) + t22413 / F::new(96.0) - t22415 / F::new(128.0) - t22417 / F::new(12.0) + F::new(11.0) / F::new(27.0) * t22420 - F::new(19.0) / F::new(108.0) * t22423 + t22425 / F::new(128.0) + F::new(19.0) / F::new(144.0) * t22428 - t22431 / F::new(64.0) - t22433 / F::new(72.0) + t22437 / F::new(24.0);
    (t22431, t22433, t22437, t22439)
}
