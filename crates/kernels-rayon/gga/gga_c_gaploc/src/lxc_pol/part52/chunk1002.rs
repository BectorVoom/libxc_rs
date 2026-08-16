//! GGA_C_GAPLOC lxc pol — lxc_pol part 52 (v4rhosigma3_17) CSE chunk 1002/1013 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part52_v4rhosigma3_17_chunk1002(t10532: f64, t10533: f64, t204: f64, t42170: f64, t42183: f64, t42187: f64, t46535: f64, t46537: f64, t46539: f64, t46542: f64, t46544: f64, t46547: f64, t46549: f64, t46553: f64, t46559: f64, t46564: f64, t46567: f64, t46570: f64, t46574: f64, t48070: f64, t50668: f64, t587: f64, t6717: f64, t6914: f64) -> f64 {
    let t50688 = t46535 + t46537 + t46539 - t46542 + t46544 + t46547 - 0.12423108009070322895e3_f64 * t6914 * t6717 * t50668 + t46549 - t46553 + t46559 + 0.55213813373645879536e2_f64 * t10532 * t10533 * t50668 - 0.18404604457881959845e2_f64 * t587 * t204 * t50668 - t48070 - t42170 + 0.38342925953920749676e1_f64 * t42183 - 0.85206502119823888169e0_f64 * t42187 + t46564 + t46567 + t46570 - t46574;
    t50688
}
