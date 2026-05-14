//! MGGA_C_RMGGAC lxc pol — lxc_pol part 17 (v4rho3sigma_8) CSE chunk 904/964 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part17_v4rho3sigma_8_chunk904<F: Float>(t2310: F, t9087: F, t2412: F, t8597: F, t1982: F, t7428: F, t9775: F, t9735: F, t2186: F, t9790: F, t46764: F, t739: F, t40699: F, t8571: F, t35691: F, t35697: F, t35699: F, t35703: F, t35705: F, t40343: F, t40350: F, t40351: F, t40354: F, t40357: F, t40459: F) -> (F,) {
    let t46992 = t9087 * t2310;
    let t46995 = t2412 * t8597;
    let t46999 = t9775 * t7428 * t1982;
    let t47004 = t9735 * t7428 * t1982;
    let t47006 = t2186 * t9790;
    let t47008 = t739 * t46764;
    let t47011 = t8571 * t40699;
    let t47013 = -0.85129199786595678796e-5 * t46992 + 0.10248087766267884742e-3 * t35691 - 0.85129199786595678796e-5 * t46995 + 0.29810146462873361018e-2 * t40343 - 0.99317399751028291929e-5 * t46999 + t40350 - 0.59590439850616975158e-4 * t40351 + 0.59590439850616975158e-4 * t40354 + t40357 - 0.19863479950205658386e-4 * t47004 + 0.99317399751028291929e-5 * t47006 - 0.39914139006212695213e-1 * t47008 - t35697 - t35699 - t35703 - 0.35220688045884876043e-2 * t35705 + 0.85129199786595678796e-5 * t47011 - t40459;
    (t47013,)
}
