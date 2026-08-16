//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 30 (v4rho3sigma_6) CSE chunk 2291/2341 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2291(t13797: f64, t17152: f64, t17161: f64, t17920: f64, t18016: f64, t18025: f64, t1920: f64, t1933: f64, t1934: f64, t23419: f64, t25585: f64, t25601: f64, t25609: f64, t378: f64, t4509: f64, t5842: f64, t5904: f64, t6735: f64, t6758: f64, t7578: f64, t83016: f64, t83080: f64, t88566: f64, t88569: f64) -> f64 {
    let t99760 = -t23419 * t18025 / 576.0_f64 + t83016 * t18016 / 576.0_f64 + 5.0_f64 / 3456.0_f64 * t23419 * t17920 - t5904 * t6758 * t378 / 288.0_f64 - t88566 + t88569 + t83080 + t1920 * t4509 * t17161 / 108.0_f64 + 7.0_f64 / 648.0_f64 * t1920 * t13797 * t17152 - 0.20186378047070195428e-3_f64 * t25601 * t25609 - 0.10093189023535097714e-3_f64 * t1933 * t1934 * t5842 * t6735 + 0.16149102437656156342e-2_f64 * t25585 * t7578;
    t99760
}
