//! GGA_C_FT97 lxc pol — lxc_pol part 23 (v4rho3sigma_8) CSE chunk 1074/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part23_v4rho3sigma_8_chunk1074<F: Float>(t10688: F, t848: F, t10491: F, t882: F, t15564: F, t15565: F, t2247: F, t172: F, t1160: F, t2372: F, t2492: F, t5132: F, t222: F, t2382: F, t226: F, t1609: F, t51: F) -> (F, F, F, F, F, F, F, F, F) {
    let t56854 = t848 * t10688;
    let t57180 = t10491 * t882;
    let t61123 = t15564 * t15565 * t2247;
    let t61128 = t15564 * t15565 * t172;
    let t65408 = t2372 * t1160;
    let t65592 = t2492 * t5132;
    let t65692 = t2382 * t222;
    let t65693 = t65692 * t226;
    let t65750 = t51 * t1609;
    (t56854, t57180, t61123, t61128, t65408, t65592, t65692, t65693, t65750)
}
