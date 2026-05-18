//! MGGA_C_RMGGAC lxc pol — lxc_pol part 15 (v4rho3sigma_6) CSE chunk 990/1110 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part15_v4rho3sigma_6_chunk990<F: Float>(t46120: F, t46148: F, t46188: F, t46205: F, t46231: F, t46264: F, t46295: F, t46313: F, t2068: F, t46129: F, t118: F, t338: F, t40891: F, t40899: F, t40908: F, t40911: F, t40918: F, t40922: F, t44083: F, t44085: F, t44089: F, t46072: F, t46076: F) -> (F, F) {
    let t46316 = t46120 + t46148 + t46188 + t46205 + t46231 + t46264 + t46295 + t46313;
    let t46320 = t2068 * t46129;
    let t46322 = -F::new(0.79828278012425390428e-1) * t118 * t46072 - F::new(0.44903406381989282115e-1) * t46076 + F::new(0.72732431077987577943e-1) * t40891 - F::new(0.21819729323396273383e0) * t40899 + t40908 - F::new(0.21819729323396273383e0) * t40911 - F::new(0.54549323308490683457e-1) * t40918 + F::new(0.36366215538993788972e0) * t40922 + F::new(0.19957069503106347607e-1) * t118 * t338 * t46316 + F::new(0.27274661654245341728e-1) * t46320 + t44083 - t44085 - t44089;
    (t46316, t46322)
}
