//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 19 (v4rho3sigma_7) CSE chunk 1269/1404 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part19_v4rho3sigma_7_chunk1269<F: Float>(t54681: F, t54716: F, t54730: F, t1211: F, t2429: F, t6926: F, t1167: F, t2494: F, t1105: F, t3324: F, t3931: F, t944: F) -> (F, F, F, F, F, F, F, F) {
    let t55962 = F::cast_from(7.0_f64) / F::cast_from(36.0_f64) * t54681;
    let t55983 = F::cast_from(7.0_f64) / F::cast_from(576.0_f64) * t54716;
    let t55987 = F::cast_from(7.0_f64) / F::cast_from(576.0_f64) * t54730;
    let t56008 = F::cast_from(12.0_f64) * t2429 * t1211 * t6926;
    let t56018 = t2494 * t1167;
    let t56027 = t1105 * t3324;
    let t56034 = t1167 * t3324;
    let t56038 = t3931 * t944;
    (t55962, t55983, t55987, t56008, t56018, t56027, t56034, t56038)
}
