//! MGGA_C_KCIS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 1003/1323 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part23_v4rho3sigma_5_chunk1003<F: Float>(t2539: F, t26398: F, t7612: F, t8522: F, t2533: F, t7630: F, t2161: F, t2770: F, t2153: F, t2626: F, t2538: F, t826: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t26399 = t26398 * t2539;
    let t26400 = F::new(2.0) * t26399;
    let t26401 = t8522 * t7612;
    let t26402 = F::new(4.0) * t26401;
    let t26409 = t2533 * t7630;
    let t26410 = F::new(2.0) * t26409;
    let t26411 = t2161 * t2770;
    let t26416 = t2153 * t2626;
    let t26417 = t2538 * t26416;
    let t26418 = F::new(2.0) * t26417;
    let t26419 = t7630 * t826;
    (t26399, t26400, t26401, t26402, t26409, t26410, t26411, t26416, t26417, t26418, t26419)
}
