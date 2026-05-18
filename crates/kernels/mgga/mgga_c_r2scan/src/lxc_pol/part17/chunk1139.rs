//! MGGA_C_R2SCAN lxc pol — lxc_pol part 17 (v4rho3sigma_7) CSE chunk 1139/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part17_v4rho3sigma_7_chunk1139<F: Float>(t40044: F, t40047: F, t40050: F, t40053: F, t40076: F, t40086: F, t40090: F, t40102: F, t40131: F, t40155: F, t40157: F, t40162: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
    let t41669 = F::new(0.27944763721877274748e0) * t40044;
    let t41670 = F::new(0.93149212406257582492e-1) * t40047;
    let t41671 = F::new(0.27944763721877274748e0) * t40050;
    let t41672 = F::new(0.93149212406257582492e-1) * t40053;
    let t41682 = F::new(0.95219938395347901946e-2) * t40076;
    let t41687 = F::new(0.93149212406257582492e-1) * t40086;
    let t41689 = F::new(0.11177905488750909899e1) * t40090;
    let t41694 = F::new(0.39029762157531132074e-1) * t40102;
    let t41709 = F::new(0.18629842481251516498e0) * t40131;
    let t41721 = F::new(0.93149212406257582492e-1) * t40155;
    let t41722 = F::new(0.46230515946956099004e0) * t40157;
    let t41725 = F::new(0.27944763721877274748e0) * t40162;
    (t41669, t41670, t41671, t41672, t41682, t41687, t41689, t41694, t41709, t41721, t41722, t41725)
}
