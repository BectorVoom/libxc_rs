//! GGA_C_FT97 lxc pol — lxc_pol part 23 (v4rho3sigma_8) CSE chunk 1054/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part23_v4rho3sigma_8_chunk1054<F: Float>(t1501: F, t5299: F, t840: F, t871: F, t5225: F, t2862: F, t1476: F, t5393: F, t1248: F, t7021: F, t4246: F, t7045: F, t31627: F, t319: F, t1255: F, t7036: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t31852 = t1501 * t5299;
    let t31854 = t840 * t871 * t31852;
    let t31857 = t1501 * t5225;
    let t31859 = t2862 * t871 * t31857;
    let t31862 = t1476 * t5393;
    let t31864 = t840 * t871 * t31862;
    let t31867 = t7021 * t1248;
    let t31869 = t840 * t871 * t31867;
    let t31873 = t840 * t4246 * t7045;
    let t31877 = t2862 * t319 * t31627;
    let t31881 = t2862 * t1255 * t7036;
    (t31852, t31854, t31857, t31859, t31862, t31864, t31867, t31869, t31873, t31877, t31881)
}
