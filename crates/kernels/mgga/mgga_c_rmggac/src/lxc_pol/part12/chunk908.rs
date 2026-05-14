//! MGGA_C_RMGGAC lxc pol — lxc_pol part 12 (v4rho3sigma_3) CSE chunk 908/951 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part12_v4rho3sigma_3_chunk908<F: Float>(t1347: F, t2408: F, t118: F, t2001: F, t352: F, t38523: F, t7720: F, t34884: F, t9118: F, t2283: F, t34881: F, t2286: F, t7939: F, t2412: F, t7914: F, t3351: F, t3352: F, t5181: F, t880: F) -> (F, F, F, F, F, F, F) {
    let t41571 = t1347 * t2408;
    let t41576 = t2001 * t118 * t38523 * t352;
    let t41577 = t7720 * t41576;
    let t41579 = t34884 * t9118;
    let t41581 = t34881 * t2283;
    let t41582 = 0.19863479950205658386e-4 * t41581;
    let t41585 = t7939 * t2286;
    let t41587 = t2412 * t7914;
    let t41591 = t3351 * t3352 * t880 * t5181;
    (t41571, t41577, t41579, t41582, t41585, t41587, t41591)
}
