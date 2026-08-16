//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 25 (v4rho3sigma_1) CSE chunk 1172/1226 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part25_v4rho3sigma_1_chunk1172(t23242: f64, t81979: f64, t10140: f64, t25: f64, t193: f64, t9458: f64, t10121: f64, t22960: f64, t46240: f64, t1081: f64, t2752: f64, t13487: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t82296 = t81979 * t23242;
    let t82313 = t25 * t10140;
    let t82320 = t193 * t9458;
    let t82323 = t25 * t10121;
    let t82330 = t22960 * t46240;
    let t83555 = t2752 * t1081;
    let t83556 = t83555 * t13487;
    (t82296, t82313, t82320, t82323, t82330, t83556)
}
