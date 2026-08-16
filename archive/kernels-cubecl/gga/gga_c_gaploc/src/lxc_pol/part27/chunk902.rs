//! GGA_C_GAPLOC lxc pol — lxc_pol part 27 (v4rho2sigma2_10) CSE chunk 902/1468 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part27_v4rho2sigma2_10_chunk902<F: Float>(t9285: F, t9287: F, t2365: F, t6520: F, t7025: F, t1415: F, t2371: F, t7030: F, t1645: F, t2349: F, t3196: F, t7014: F) -> (F, F, F, F, F, F, F) {
    let t9289 = F::cast_from(0.29792074959875355558e-1_f64) * t9285 * t9287;
    let t9294 = t2365 * t6520;
    let t9296 = F::cast_from(0.29792074959875355558e-1_f64) * t7025 * t9294;
    let t9305 = t1415 * t2371;
    let t9307 = F::cast_from(0.29792074959875355558e-1_f64) * t9305 * t7030;
    let t9333 = t1645 * t2349;
    let t9362 = t7014 * t3196;
    (t9289, t9294, t9296, t9305, t9307, t9333, t9362)
}
