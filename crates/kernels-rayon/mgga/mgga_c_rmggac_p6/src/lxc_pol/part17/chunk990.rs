//! MGGA_C_RMGGAC lxc pol — lxc_pol part 17 (v4rho3sigma_8) CSE chunk 990/1111 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part17_v4rho3sigma_8_chunk990(t46120: f64, t46148: f64, t46188: f64, t46205: f64, t46231: f64, t46264: f64, t46295: f64, t46313: f64, t2068: f64, t46129: f64, t118: f64, t338: f64, t40891: f64, t40899: f64, t40908: f64, t40911: f64, t40918: f64, t40922: f64, t44083: f64, t44085: f64, t44089: f64, t46072: f64, t46076: f64) -> (f64, f64) {
    let t46316 = t46120 + t46148 + t46188 + t46205 + t46231 + t46264 + t46295 + t46313;
    let t46320 = t2068 * t46129;
    let t46322 = -0.79828278012425390428e-1_f64 * t118 * t46072 - 0.44903406381989282115e-1_f64 * t46076 + 0.72732431077987577943e-1_f64 * t40891 - 0.21819729323396273383e0_f64 * t40899 + t40908 - 0.21819729323396273383e0_f64 * t40911 - 0.54549323308490683457e-1_f64 * t40918 + 0.36366215538993788972e0_f64 * t40922 + 0.19957069503106347607e-1_f64 * t118 * t338 * t46316 + 0.27274661654245341728e-1_f64 * t46320 + t44083 - t44085 - t44089;
    (t46316, t46322)
}
