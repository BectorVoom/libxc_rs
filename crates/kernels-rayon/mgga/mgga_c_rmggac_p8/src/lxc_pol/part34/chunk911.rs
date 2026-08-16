//! MGGA_C_RMGGAC lxc pol — lxc_pol part 34 (v4rho2sigma2_7) CSE chunk 911/1097 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part34_v4rho2sigma2_7_chunk911(t76310: f64, t30526: f64, t3065: f64, t556: f64, t14290: f64, t1612: f64, t1627: f64, t3080: f64, t25820: f64, t8377: f64, t27101: f64, t75336: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t76311 = 0.15965655602485078085e0_f64 * t76310;
    let t76313 = t30526 * t3065 * t556;
    let t76315 = t14290 * t1612;
    let t76317 = t3080 * t1627;
    let t76319 = 0.17961362552795712846e0_f64 * t25820 * t76317;
    let t76320 = t3080 * t8377;
    let t76322 = 0.11974241701863808564e0_f64 * t27101 * t76320;
    let t76323 = t75336 * t3065;
    (t76311, t76313, t76315, t76317, t76319, t76320, t76322, t76323)
}
