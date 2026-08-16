//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 636/1383 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk636(t3844: f64, t885: f64, t1436: f64, t2577: f64, t884: f64, t2455: f64, t2581: f64, t3746: f64, t3751: f64, t3756: f64, t3760: f64, t318: f64) -> (f64, f64, f64, f64, f64) {
    let t3845 = t3844 * t885;
    let t3848 = t1436 * t2577;
    let t3849 = t3848 * t884;
    let t3857 = t2581 + 0.30902777777777777778e-2_f64 * t2455 + 0.30902777777777777778e-2_f64 * t3746 - 0.61805555555555555555e-2_f64 * t3751 + 0.18541666666666666667e-1_f64 * t3756 - 0.92708333333333333333e-2_f64 * t3760;
    let t3858 = t3857 * t318;
    (t3845, t3848, t3849, t3857, t3858)
}
