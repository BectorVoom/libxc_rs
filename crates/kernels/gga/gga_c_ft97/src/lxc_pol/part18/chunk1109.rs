//! GGA_C_FT97 lxc pol — lxc_pol part 18 (v4rho3sigma_3) CSE chunk 1109/1396 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part18_v4rho3sigma_3_chunk1109<F: Float>(t1882: F, t23179: F, t5641: F, t8232: F, t5650: F, t5661: F, t5712: F, t23367: F, t23363: F, t463: F, t5704: F, t38953: F, t5719: F, t23251: F, t8392: F, t23355: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t93579 = t1882 * t23179;
    let t93609 = t8232 * t5641;
    let t93612 = t8232 * t5650;
    let t93621 = t8232 * t5661;
    let t93630 = t8232 * t5712;
    let t93632 = t1882 * t23367;
    let t93634 = t1882 * t23363;
    let t93636 = t463 * t5704;
    let t93647 = t38953 * t5719;
    let t93649 = t8392 * t23251;
    let t93656 = t1882 * t23355;
    (t93579, t93609, t93612, t93621, t93630, t93632, t93634, t93636, t93647, t93649, t93656)
}
