//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 19 (v4rho3sigma_7) CSE chunk 1093/1222 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part19_v4rho3sigma_7_chunk1093<F: Float>(t51201: F, t51222: F, t51350: F, t6684: F, t3249: F, t6238: F, t899: F, t923: F, t2209: F, t4026: F, t863: F, t1135: F, t9246: F, t2134: F, t28139: F, t850: F) -> (F, F, F, F, F, F, F, F) {
    let t54026 = 119.0 / 1728.0 * t51201;
    let t54038 = 35.0 / 216.0 * t51222;
    let t54047 = t6684 * t51350;
    let t54052 = t899 * t6238 * t923 * t3249;
    let t54055 = t863 * t4026 * t2209;
    let t54071 = t9246 * t1135;
    let t54072 = t2134 * t54071;
    let t54079 = t850 * t28139;
    (t54026, t54038, t54047, t54052, t54055, t54071, t54072, t54079)
}
