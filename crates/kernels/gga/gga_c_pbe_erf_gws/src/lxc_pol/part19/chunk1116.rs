//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 19 (v4rho3sigma_7) CSE chunk 1116/1222 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part19_v4rho3sigma_7_chunk1116<F: Float>(t2242: F, t4230: F, t15027: F, t9270: F, t15089: F, t4414: F, t14924: F, t54681: F, t54716: F, t54730: F, t1211: F, t2429: F, t6926: F, t1167: F, t2494: F, t1105: F, t3324: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t55904 = t2242 * t4230;
    let t55918 = 7.0 / 72.0 * t9270 * t15027;
    let t55936 = 7.0 / 72.0 * t4414 * t15089;
    let t55942 = 7.0 / 72.0 * t4414 * t14924;
    let t55962 = 7.0 / 36.0 * t54681;
    let t55983 = 7.0 / 576.0 * t54716;
    let t55987 = 7.0 / 576.0 * t54730;
    let t56008 = 12.0 * t2429 * t1211 * t6926;
    let t56018 = t2494 * t1167;
    let t56027 = t1105 * t3324;
    (t55904, t55918, t55936, t55942, t55962, t55983, t55987, t56008, t56018, t56027)
}
