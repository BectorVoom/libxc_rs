//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2435/2721 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2435(t2792: f64, t4396: f64, t5726: f64, t1557: f64, t17422: f64, t10655: f64, t21318: f64, t1556: f64, t2842: f64, t60745: f64, t17520: f64, t4395: f64) -> (f64, f64, f64, f64, f64) {
    let t69302 = 6.0_f64 * t2792 * t4396 * t5726;
    let t69305 = 6.0_f64 * t2792 * t1557 * t17422;
    let t69307 = 0.48245938496077605201e2_f64 * t10655 * t21318;
    let t69310 = 0.48245938496077605201e2_f64 * t2842 * t60745 * t1556;
    let t69313 = 0.48245938496077605201e2_f64 * t2842 * t17520 * t4395;
    (t69302, t69305, t69307, t69310, t69313)
}
