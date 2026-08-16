//! MGGA_C_KCIS lxc pol — lxc_pol part 5 (v3rho3_2) CSE chunk 1162/1419 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part5_v3rho3_2_chunk1162<F: Float>(t6620: F, t9415: F, t3200: F, t2822: F, t6501: F, t1662: F, t4984: F, t9517: F, t1767: F, t3217: F, t4813: F, t3202: F) -> (F, F, F, F) {
    let t19565 = t9415 * t6620;
    let t19566 = t3200 * t19565;
    let t19569 = t2822 * t6501;
    let t19571 = t1662 * t4984;
    let t19572 = t9517 * t19571;
    let t19573 = t3200 * t19572;
    let t19575 = t3217 * t1767;
    let t19576 = t19575 * t4813;
    let t19577 = t3202 * t19576;
    (t19566, t19569, t19573, t19577)
}
