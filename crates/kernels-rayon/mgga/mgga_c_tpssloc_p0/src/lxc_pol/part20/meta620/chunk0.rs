//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2233/2712 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2233(t40748: f64, t40760: f64, t40764: f64, t40766: f64, t46282: f64, t46284: f64, t46286: f64, t46287: f64, t46288: f64, t46292: f64, t46293: f64, t2379: f64, t868: f64) -> (f64, f64) {
    let t46294 = t46282 + t46284 + t46286 + t40748 + t46287 + t40760 - t46288 + t46292 + t40764 + t40766 + t46293;
    let t46298 = t2379 * t868;
    (t46294, t46298)
}
