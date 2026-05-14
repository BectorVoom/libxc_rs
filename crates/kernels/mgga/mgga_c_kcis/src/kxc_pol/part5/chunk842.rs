//! MGGA_C_KCIS kxc pol — kxc_pol part 5 (v3rho3_2) CSE chunk 842/1260 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_kxc_pol_part5_v3rho3_2_chunk842<F: Float>(t2568: F, t8561: F, t126: F, t691: F, t2314: F, t4: F, t789: F, t15: F, t26: F, t92: F, t160: F, t3: F, t20: F, t725: F, t2316: F, t2469: F) -> (F, F, F, F, F, F, F, F, F) {
    let t8562 = t2568 * t8561;
    let t8565 = t126 * t691;
    let t8566 = t8565 * t2314;
    let t8567 = t789 * t4;
    let t8572 = 1.0 / t15 / t26 / 4.0;
    let t8573 = t8572 * t92;
    let t8574 = t3 * t160;
    let t8578 = t2314 * t725 * t20;
    let t8581 = t2316 * t2469;
    (t8562, t8565, t8566, t8567, t8572, t8573, t8574, t8578, t8581)
}
