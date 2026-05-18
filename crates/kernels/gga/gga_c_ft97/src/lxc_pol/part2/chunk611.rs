//! GGA_C_FT97 lxc pol — lxc_pol part 2 (v3rho3_1) CSE chunk 611/1007 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part2_v3rho3_1_chunk611<F: Float>(t2360: F, t312: F, t3886: F, t2881: F, t1242: F, t681: F, t89: F, t1225: F, t1882: F, t1212: F, t840: F, t882: F) -> (F, F, F, F, F, F) {
    let t4265 = t312 * t2360;
    let t4266 = t4265 * t3886;
    let t4267 = t2881 * t4266;
    let t4271 = t89 * t681 * t1242;
    let t4273 = t1882 * t1225;
    let t4276 = t840 * t882 * t1212;
    (t4265, t4266, t4267, t4271, t4273, t4276)
}
