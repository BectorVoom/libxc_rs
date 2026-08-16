//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 57 (v4rho2sigma2_13) CSE chunk 741/1049 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part57_v4rho2sigma2_13_chunk741(t112: f64, t7758: f64, t12461: f64, t2094: f64, t193: f64, t200: f64, t2056: f64, t25049: f64, t25277: f64, t25077: f64, t25080: f64, t25140: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t26523 = t7758 * t112;
    let t26558 = t2094 * t12461;
    let t26563 = t193 * t200 * t2056;
    let t26591 = 0.38381794893125283518e-1_f64 * t25049;
    let t26613 = 0.38381794893125283518e-1_f64 * t25277;
    let t26619 = 7.0_f64 / 288.0_f64 * t25077;
    let t26621 = 7.0_f64 / 1152.0_f64 * t25080;
    let t26644 = 7.0_f64 / 72.0_f64 * t25140;
    (t26523, t26558, t26563, t26591, t26613, t26619, t26621, t26644)
}
