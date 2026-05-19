//! MGGA_C_KCIS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 1241/1397 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part26_v4rho3sigma_8_chunk1241<F: Float>(t28519: F, t4142: F, t17287: F, t491: F, t990: F, t1593: F, t28352: F, t498: F, t27369: F, t12234: F, t1938: F, t28419: F, t52649: F, t7908: F) -> (F, F, F, F, F, F, F) {
    let t98104 = t4142 * t28519;
    let t98105 = F::cast_from(0.22109259259259259258e-2_f64) * t98104;
    let t98119 = t17287 * t491 * t990;
    let t98137 = t1593 * t498 * t28352;
    let t98138 = t27369 * t98137;
    let t98144 = t12234 * t1938;
    let t98150 = t7908 * t52649 * t28419;
    (t98104, t98105, t98119, t98137, t98138, t98144, t98150)
}
