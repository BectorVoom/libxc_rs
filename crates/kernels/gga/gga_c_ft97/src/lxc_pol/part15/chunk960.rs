//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 960/1222 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk960<F: Float>(t1127: F, t3724: F, t66115: F, t21222: F, t25: F, t1113: F, t13580: F, t18132: F, t18089: F, t226: F, t21157: F, t1109: F, t5025: F) -> (F, F, F, F, F, F) {
    let t79317 = t3724 * t66115 * t1127;
    let t79341 = t21222 * t25;
    let t79373 = t13580 * t18132 * t1113;
    let t79402 = t18089 * t226;
    let t79423 = t21157 * t25;
    let t79430 = t1109 * t5025;
    (t79317, t79341, t79373, t79402, t79423, t79430)
}
