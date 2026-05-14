//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 20 (v4rho3sigma_8) CSE chunk 1167/1210 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part20_v4rho3sigma_8_chunk1167<F: Float>(t2080: F, t3107: F, t12044: F, t14092: F, t38537: F, t14093: F, t3803: F, t852: F, t6341: F, t14064: F, t3805: F, t1184: F, t12000: F, t11535: F, t14031: F, t3128: F, t54359: F) -> (F, F, F, F, F, F, F) {
    let t57026 = t2080 * t3107;
    let t57028 = t57026 * t14092 * t12044;
    let t57030 = t2080 * t38537;
    let t57031 = t57030 * t14093;
    let t57034 = t2080 * t3803 * t852;
    let t57036 = t57034 * t14092 * t6341;
    let t57038 = t3805 * t14064;
    let t57040 = t1184 * t12000;
    let t57042 = t14031 * t11535;
    let t57044 = t3128 * t54359;
    (t57028, t57031, t57036, t57038, t57040, t57042, t57044)
}
