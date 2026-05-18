//! GGA_C_GAPLOC lxc pol — lxc_pol part 42 (v4rhosigma3_7) CSE chunk 1002/1012 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part42_v4rhosigma3_7_chunk1002<F: Float>(t10532: F, t10533: F, t204: F, t42170: F, t42183: F, t42187: F, t46535: F, t46537: F, t46539: F, t46542: F, t46544: F, t46547: F, t46549: F, t46553: F, t46559: F, t46564: F, t46567: F, t46570: F, t46574: F, t48070: F, t50668: F, t587: F, t6717: F, t6914: F) -> F {
    let t50688 = t46535 + t46537 + t46539 - t46542 + t46544 + t46547 - F::new(0.12423108009070322895e3) * t6914 * t6717 * t50668 + t46549 - t46553 + t46559 + F::new(0.55213813373645879536e2) * t10532 * t10533 * t50668 - F::new(0.18404604457881959845e2) * t587 * t204 * t50668 - t48070 - t42170 + F::new(0.38342925953920749676e1) * t42183 - F::new(0.85206502119823888169e0) * t42187 + t46564 + t46567 + t46570 - t46574;
    t50688
}
