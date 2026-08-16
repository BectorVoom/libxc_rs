//! MGGA_C_RMGGAC lxc pol — lxc_pol part 37 (v4rho2sigma2_10) CSE chunk 677/1128 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part37_v4rho2sigma2_10_chunk677(t14255: f64, t68524: f64, t14018: f64, t7715: f64, t3119: f64, t3899: f64, t464: f64, t14024: f64, t14122: f64, t14127: f64, t14130: f64, t68489: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t68525 = t68524 * t14255;
    let t68526 = 0.29085809927086856922e-4_f64 * t68525;
    let t68527 = t14018 * t7715;
    let t68528 = t68527 * t3119;
    let t68536 = t464 * t3899;
    let t68537 = t68536 * t14024;
    let t68538 = t14122 * t68537;
    let t68539 = t68538 * t14127;
    let t68540 = 0.16351352353374609375e-5_f64 * t68539;
    let t68541 = t14130 * t68489;
    (t68526, t68527, t68528, t68536, t68538, t68540, t68541)
}
