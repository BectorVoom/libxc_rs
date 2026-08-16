//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 709/1184 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk709<F: Float>(t1248: F, t6260: F, t840: F, t871: F, t1476: F, t4299: F, t1255: F, t2862: F, t6278: F, t4246: F, t6287: F, t7036: F, t882: F) -> (F, F, F, F, F, F, F) {
    let t29302 = t6260 * t1248;
    let t29304 = t840 * t871 * t29302;
    let t29307 = t1476 * t4299;
    let t29309 = t840 * t871 * t29307;
    let t29313 = t2862 * t1255 * t6278;
    let t29317 = t840 * t4246 * t6287;
    let t29321 = t2862 * t882 * t7036;
    (t29302, t29304, t29307, t29309, t29313, t29317, t29321)
}
