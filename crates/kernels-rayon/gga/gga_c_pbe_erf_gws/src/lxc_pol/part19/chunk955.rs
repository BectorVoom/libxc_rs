//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 19 (v4rho3sigma_7) CSE chunk 955/1404 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part19_v4rho3sigma_7_chunk955(t10777: f64, t418: f64, t1856: f64, t10756: f64, t10758: f64, t10760: f64, t10763: f64, t10771: f64, t10774: f64, t25: f64, t4941: f64, t5241: f64, t5256: f64, t5271: f64, t7335: f64, t7364: f64, t7374: f64, t7376: f64, t7379: f64, t7380: f64) -> (f64, f64) {
    let t10778 = t10777 * t418;
    let t10779 = t1856 * t10778;
    let t10782 = 0.44444444444444444445e-2_f64 * t10756 + 0.14814814814814814815e-2_f64 * t10758 - 0.88888888888888888887e-2_f64 * t10760 - 0.66666666666666666667e-2_f64 * t25 * t10763 - 0.15996296296296296296e-1_f64 * t4941 - t5241 + t7335 - t7364 - t5271 - 0.31992592592592592592e-1_f64 * t7374 + 0.47988888888888888888e-1_f64 * t7376 + t7379 - 0.47988888888888888888e-1_f64 * t7380 - 0.74074074074074074073e-2_f64 * t5256 + 0.13333333333333333333e-1_f64 * t25 * t10771 - 0.22222222222222222222e-2_f64 * t25 * t10774 + 0.13333333333333333333e-1_f64 * t25 * t10779;
    (t10778, t10782)
}
