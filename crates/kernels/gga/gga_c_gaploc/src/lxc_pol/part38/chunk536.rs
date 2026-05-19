//! GGA_C_GAPLOC lxc pol — lxc_pol part 38 (v4rhosigma3_3) CSE chunk 536/1003 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part38_v4rhosigma3_3_chunk536<F: Float>(t10254: F, t2787: F, t6519: F, t2343: F, t1063: F, t2312: F, t3344: F, t2321: F, t2822: F, t882: F, t2765: F, t6750: F) -> (F, F, F, F, F, F, F, F) {
    let t10255 = F::cast_from(0.11856252764865062333e-2_f64) * t10254;
    let t10256 = t2787 * t6519;
    let t10257 = t2343 * t10256;
    let t10259 = F::cast_from(0.56910013271352299198e-1_f64) * t1063 * t10257;
    let t10260 = t2312 * t3344;
    let t10261 = F::cast_from(0.11856252764865062333e-2_f64) * t10260;
    let t10262 = t2822 * t2321;
    let t10263 = t882 * t10262;
    let t10264 = F::cast_from(0.11856252764865062333e-2_f64) * t10263;
    let t10265 = t2765 * t6750;
    (t10255, t10256, t10259, t10260, t10261, t10263, t10264, t10265)
}
