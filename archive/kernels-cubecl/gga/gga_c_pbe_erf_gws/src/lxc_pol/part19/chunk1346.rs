//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 19 (v4rho3sigma_7) CSE chunk 1346/1404 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part19_v4rho3sigma_7_chunk1346<F: Float>(t11365: F, t51898: F, t12215: F, t3965: F, t15317: F, t51682: F, t2409: F, t36089: F, t3959: F, t14001: F, t15331: F, t1178: F, t12169: F, t371: F, t3983: F) -> (F, F, F, F, F, F) {
    let t57685 = t51898 * t11365;
    let t57687 = t3965 * t12215;
    let t57689 = t51682 * t15317;
    let t57694 = t3959 * t2409 * t36089;
    let t57696 = t14001 * t15331;
    let t57700 = t3983 * t371 * t1178 * t12169;
    (t57685, t57687, t57689, t57694, t57696, t57700)
}
