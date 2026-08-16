//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 569/1527 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk569(t1314: f64, t792: f64, t116: f64, t534: f64, t212: f64, t2586: f64, t2600: f64, t541: f64, t1337: f64, t551: f64) -> (f64, f64, f64, f64, f64) {
    let t3739 = t792 * t1314;
    let t3748 = t534 * t116;
    let t3749 = t3748 * t212;
    let t3751 = 0.83333333333333333332e-3_f64 * t2586 * t3749;
    let t3762 = 35.0_f64 / 432.0_f64 * t2600 * t541;
    let t3787 = 1.0_f64 / t1337 / t551;
    (t3739, t3749, t3751, t3762, t3787)
}
