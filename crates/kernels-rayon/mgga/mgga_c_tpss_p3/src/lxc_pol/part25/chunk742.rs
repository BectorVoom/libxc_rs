//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 742/1383 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk742(t4891: f64, t885: f64, t2557: f64, t2564: f64, t3746: f64, t3795: f64, t4828: f64, t4832: f64, t4836: f64, t4848: f64, t4855: f64, t4861: f64, t4863: f64, t4867: f64, t4870: f64, t4873: f64) -> (f64, f64) {
    let t4892 = t4891 * t885;
    let t4907 = -0.17648625e1_f64 * t4848 + 0.3529725e1_f64 * t4855 + t2557 + 0.34431666666666666666e0_f64 * t3746 - 0.34431666666666666667e0_f64 * t4828 + 0.103295e1_f64 * t4832 - 0.516475e0_f64 * t4836 + 0.31558125e0_f64 * t4861 + 0.6311625e0_f64 * t4863 + t2564 + 0.13892666666666666667e0_f64 * t3795 - 0.34731666666666666667e-1_f64 * t4867 + 0.20839e0_f64 * t4870 - 0.104195e0_f64 * t4873;
    (t4892, t4907)
}
