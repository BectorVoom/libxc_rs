//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 17 (v4rho3sigma_5) CSE chunk 1343/1352 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part17_v4rho3sigma_5_chunk1343<F: Float>(t4135: F, t51966: F, t22410: F, t2409: F, t3959: F, t22192: F, t3965: F, t9220: F, t26885: F, t2242: F, t4185: F, t1146: F, t13987: F) -> (F, F, F, F, F, F, F) {
    let t54621 = t51966 * t4135;
    let t54624 = t3959 * t2409 * t22410;
    let t54627 = t3965 * t2409 * t22192;
    let t54629 = t3959 * t9220;
    let t54636 = t3965 * t2409 * t26885;
    let t54639 = t2242 * t4185;
    let t54641 = t13987 * t1146;
    (t54621, t54624, t54627, t54629, t54636, t54639, t54641)
}
