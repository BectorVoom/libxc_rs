//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 57 (v4rho2sigma2_13) CSE chunk 798/1049 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part57_v4rho2sigma2_13_chunk798(t1058: f64, t1610: f64, t1953: f64, t23327: f64, t23601: f64, t23633: f64, t25530: f64, t25563: f64, t28638: f64, t28642: f64, t28648: f64, t28653: f64, t28657: f64, t28660: f64, t28663: f64, t28667: f64, t28671: f64, t28674: f64, t3186: f64, t5903: f64, t6687: f64, t7622: f64) -> f64 {
    let t28677 = 0.36554090374405031923e-2_f64 * t6687 * t28638 + t1058 * t28642 + 0.54831135561607547884e-2_f64 * t25530 + t5903 * t1953 + 2.0_f64 * t1610 * t7622 - 0.54831135561607547884e-2_f64 * t23327 * t28648 + 0.54831135561607547884e-2_f64 * t23633 * t28653 + 0.18277045187202515961e-2_f64 * t25563 - 0.82246703342411321825e-2_f64 * t6687 * t28657 - 0.82246703342411321825e-2_f64 * t6687 * t28660 - 0.16449340668482264365e-1_f64 * t6687 * t28663 + 0.16449340668482264365e-1_f64 * t23601 * t28667 - 0.82246703342411321825e-2_f64 * t23601 * t28671 + 2.0_f64 * t3186 * t28674;
    t28677
}
