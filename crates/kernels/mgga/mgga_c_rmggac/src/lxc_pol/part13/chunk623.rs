//! MGGA_C_RMGGAC lxc pol — lxc_pol part 13 (v4rho3sigma_4) CSE chunk 623/1127 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part13_v4rho3sigma_4_chunk623<F: Float>(t2205: F, t2604: F, t2244: F, t275: F, t2262: F, t504: F, t699: F, t798: F, t903: F, t2211: F, t4048: F, t739: F) -> (F, F, F, F, F, F, F) {
    let t8206 = t2604 * t2205;
    let t8207 = F::cast_from(0.11974241701863808564e0_f64) * t8206;
    let t8208 = t275 * t2244;
    let t8210 = t504 * t2262;
    let t8211 = F::cast_from(0.39914139006212695214e-1_f64) * t8210;
    let t8212 = t699 * t798;
    let t8213 = t903 * t8212;
    let t8214 = F::cast_from(0.35922725105591425692e0_f64) * t8213;
    let t8215 = t2211 * t4048;
    let t8216 = t739 * t8215;
    (t8207, t8208, t8211, t8212, t8214, t8215, t8216)
}
