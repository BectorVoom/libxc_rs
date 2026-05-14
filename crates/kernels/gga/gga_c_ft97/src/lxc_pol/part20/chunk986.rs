//! GGA_C_FT97 lxc pol — lxc_pol part 20 (v4rho3sigma_5) CSE chunk 986/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part20_v4rho3sigma_5_chunk986<F: Float>(t2749: F, t848: F, t10491: F, t882: F, t10478: F, t15564: F, t15565: F, t2247: F, t172: F, t1160: F, t2372: F, t1127: F, t2428: F, t13411: F, t2417: F, t17818: F) -> (F, F, F, F, F, F, F, F, F) {
    let t57032 = t848 * t2749;
    let t57180 = t10491 * t882;
    let t57186 = t10478 * t882;
    let t61123 = t15564 * t15565 * t2247;
    let t61128 = t15564 * t15565 * t172;
    let t65408 = t2372 * t1160;
    let t65676 = t1127 * t2428;
    let t65684 = t13411 * t2417;
    let t65685 = t65684 * t17818;
    (t57032, t57180, t57186, t61123, t61128, t65408, t65676, t65684, t65685)
}
