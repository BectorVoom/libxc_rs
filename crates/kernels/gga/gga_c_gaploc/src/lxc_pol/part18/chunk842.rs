//! GGA_C_GAPLOC lxc pol — lxc_pol part 18 (v4rho2sigma2_1) CSE chunk 842/1268 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part18_v4rho2sigma2_1_chunk842<F: Float>(t9285: F, t9287: F, t2365: F, t6520: F, t7025: F, t1415: F, t2371: F, t7030: F, t1645: F, t2349: F, t3196: F, t7014: F, t2488: F, t9278: F, t2487: F, t2344: F, t2465: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t9289 = 0.29792074959875355558e-1 * t9285 * t9287;
    let t9294 = t2365 * t6520;
    let t9296 = 0.29792074959875355558e-1 * t7025 * t9294;
    let t9305 = t1415 * t2371;
    let t9307 = 0.29792074959875355558e-1 * t9305 * t7030;
    let t9333 = t1645 * t2349;
    let t9362 = t7014 * t3196;
    let t9363 = 0.38342925953920749676e0 * t9362;
    let t9364 = t2488 * t9278;
    let t9365 = t2487 * t9364;
    let t9366 = 0.38342925953920749676e0 * t9365;
    let t9367 = t2465 * t2344;
    (t9289, t9294, t9296, t9305, t9307, t9333, t9363, t9364, t9366, t9367)
}
