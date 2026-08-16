//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 31 (v4rho3sigma_7) CSE chunk 1716/2041 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1716(t24246: f64, t24250: f64, t25246: f64, t25259: f64, t28323: f64, t28331: f64, t28335: f64, t28339: f64, t28343: f64, t28347: f64, t28997: f64, t29000: f64, t4166: f64, t7837: f64, t812: f64) -> f64 {
    let t29009 = -0.16449340668482264365e-1_f64 * t28323 + 0.16449340668482264365e-1_f64 * t25246 - 0.16449340668482264365e-1_f64 * t25259 - 0.3289868133696452873e-1_f64 * t28331 - 2.0_f64 * t812 * t28997 + t24246 + 2.0_f64 * t812 * t29000 - 2.0_f64 * t4166 * t7837 + t24250 + 0.16449340668482264365e-1_f64 * t28335 + 0.6579736267392905746e-1_f64 * t28339 + 0.9869604401089358619e-1_f64 * t28343 - 0.6579736267392905746e-1_f64 * t28347;
    t29009
}
