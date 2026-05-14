//! GGA_C_FT97 lxc pol — lxc_pol part 23 (v4rho3sigma_8) CSE chunk 1142/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part23_v4rho3sigma_8_chunk1142<F: Float>(t1882: F, t28441: F, t28158: F, t46862: F, t28455: F, t8392: F, t2492: F, t6907: F, t28184: F, t28434: F, t1443: F, t2372: F, t6863: F, t8232: F, t2486: F, t6154: F) -> (F, F, F, F, F, F, F, F, F) {
    let t110293 = 4.0 / 9.0 * t1882 * t28441;
    let t110294 = t46862 * t28158;
    let t110364 = 2.0 / 27.0 * t8392 * t28455;
    let t110369 = t2492 * t6907;
    let t110380 = 4.0 / 27.0 * t8392 * t28184;
    let t110400 = 4.0 / 9.0 * t1882 * t28434;
    let t110401 = t2372 * t1443;
    let t110420 = t8232 * t6863;
    let t110438 = t2486 * t6154;
    (t110293, t110294, t110364, t110369, t110380, t110400, t110401, t110420, t110438)
}
