//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 19 (v4rho3sigma_7) CSE chunk 1314/1404 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part19_v4rho3sigma_7_chunk1314<F: Float>(t14093: F, t57030: F, t2080: F, t3803: F, t852: F, t14092: F, t6341: F, t14064: F, t3805: F, t1184: F, t12000: F, t11535: F, t14031: F) -> (F, F, F, F, F) {
    let t57031 = t57030 * t14093;
    let t57034 = t2080 * t3803 * t852;
    let t57036 = t57034 * t14092 * t6341;
    let t57038 = t3805 * t14064;
    let t57040 = t1184 * t12000;
    let t57042 = t14031 * t11535;
    (t57031, t57036, t57038, t57040, t57042)
}
