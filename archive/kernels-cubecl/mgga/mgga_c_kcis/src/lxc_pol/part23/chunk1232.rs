//! MGGA_C_KCIS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 1232/1323 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part23_v4rho3sigma_5_chunk1232<F: Float>(t12234: F, t531: F, t1650: F, t3715: F, t5709: F, t1394: F, t16700: F, t27387: F, t28519: F, t4142: F, t15919: F, t28503: F) -> (F, F, F, F, F) {
    let t98084 = t12234 * t531;
    let t98087 = t5709 * t98084 * t1650 * t3715;
    let t98102 = t1394 * t27387 * t16700;
    let t98104 = t4142 * t28519;
    let t98105 = F::cast_from(0.22109259259259259258e-2_f64) * t98104;
    let t98107 = t1394 * t28503 * t15919;
    (t98087, t98102, t98104, t98105, t98107)
}
