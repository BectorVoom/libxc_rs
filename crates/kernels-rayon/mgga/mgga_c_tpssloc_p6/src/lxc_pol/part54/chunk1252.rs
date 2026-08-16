//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 54 (v4rho2sigma2_10) CSE chunk 1252/1484 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part54_v4rho2sigma2_10_chunk1252(t607: f64, t63: f64, t193: f64, t201: f64, t7109: f64, t10143: f64, t111: f64, t7415: f64, t25: f64, t40772: f64, t1408: f64, t2752: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t84186 = t607 * t63;
    let t84797 = t193 * t201 * t7109;
    let t84800 = t7109 * t10143;
    let t85416 = t7415 * t111;
    let t86716 = t40772 * t25;
    let t86721 = t2752 * t1408;
    (t84186, t84797, t84800, t85416, t86716, t86721)
}
