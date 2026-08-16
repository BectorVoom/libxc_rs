//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 16 (v4rho3sigma_4) CSE chunk 1356/1361 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part16_v4rho3sigma_4_chunk1356<F: Float>(t2242: F, t4230: F, t15027: F, t9270: F, t15089: F, t4414: F, t14924: F, t1205: F, t14258: F, t14264: F, t14952: F, t19631: F, t2182: F, t2376: F, t2408: F, t2409: F, t26654: F, t3066: F, t3067: F, t3207: F, t34963: F, t4088: F, t4207: F, t4227: F, t51979: F, t52582: F, t54629: F, t54636: F, t6781: F, t8589: F, t9688: F) -> F {
    let t55904 = t2242 * t4230;
    let t55918 = F::cast_from(7.0_f64) / F::cast_from(72.0_f64) * t9270 * t15027;
    let t55936 = F::cast_from(7.0_f64) / F::cast_from(72.0_f64) * t4414 * t15089;
    let t55942 = F::cast_from(7.0_f64) / F::cast_from(72.0_f64) * t4414 * t14924;
    let t55945 = -F::cast_from(35.0_f64) / F::cast_from(432.0_f64) * t55904 - t54629 / F::cast_from(12.0_f64) - t52582 + t3066 * t2409 * t3067 * t1205 * t9688 / F::cast_from(48.0_f64) - t3207 * t2409 * t2376 * t4227 * t2182 / F::cast_from(16.0_f64) - t55918 - t3066 * t2409 * t34963 * t14264 / F::cast_from(16.0_f64) + t2408 * t2409 * t19631 * t4207 / F::cast_from(48.0_f64) + t2408 * t2409 * t8589 * t14258 / F::cast_from(24.0_f64) + t2408 * t2409 * t6781 * t14952 / F::cast_from(24.0_f64) - t55936 + t2408 * t2409 * t26654 * t4088 / F::cast_from(24.0_f64) - t55942 - t54636 / F::cast_from(48.0_f64) + F::cast_from(7.0_f64) / F::cast_from(72.0_f64) * t51979;
    t55945
}
