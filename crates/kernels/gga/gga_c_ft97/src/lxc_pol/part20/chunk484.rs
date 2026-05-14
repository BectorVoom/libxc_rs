//! GGA_C_FT97 lxc pol — lxc_pol part 20 (v4rho3sigma_5) CSE chunk 484/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part20_v4rho3sigma_5_chunk484<F: Float>(t6360: F, t684: F, t2881: F, t1501: F, t824: F, t840: F, t871: F, t2749: F, t296: F, t875: F, t2843: F, t6315: F, t6332: F, t6312: F, t6321: F, t6325: F, t6329: F, t6337: F, t6341: F, t6345: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
    let t6361 = t6360 * t684;
    let t6362 = t2881 * t6361;
    let t6365 = t1501 * t824;
    let t6367 = t840 * t871 * t6365;
    let t6370 = t2749 * t1501;
    let t6371 = t296 * t6370;
    let t6374 = t1501 * t875;
    let t6375 = t2843 * t6374;
    let t6376 = t296 * t6375;
    let t6380 = t6315 / 6.0;
    let t6383 = t6332 / 3.0;
    let t6386 = t6312 / 4.0 + t6380 + t6321 / 6.0 + t6325 - t6329 / 2.0 + t6383 + t6337 / 3.0 + 2.0 * t6341 - t6345;
    (t6361, t6362, t6365, t6367, t6370, t6371, t6374, t6375, t6376, t6380, t6383, t6386)
}
