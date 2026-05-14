//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 16 (v4rho3sigma_4) CSE chunk 970/1184 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part16_v4rho3sigma_4_chunk970<F: Float>(t13919: F, t2258: F, t13917: F, t331: F, t816: F, t2080: F, t2084: F, t833: F, t2409: F, t6133: F, t3959: F, t326: F, t837: F, t867: F) -> (F, F, F, F, F, F, F, F) {
    let t13920 = t13919 * t2258;
    let t13921 = t13917 * t13920;
    let t13942 = t816 * t331;
    let t13944 = t2080 * t2084 * t13942;
    let t13945 = t13944 * t833;
    let t13949 = t2409 * t6133;
    let t13950 = t3959 * t13949;
    let t13952 = t326 * t837;
    let t13953 = t13952 * t867;
    (t13920, t13921, t13944, t13945, t13949, t13950, t13952, t13953)
}
