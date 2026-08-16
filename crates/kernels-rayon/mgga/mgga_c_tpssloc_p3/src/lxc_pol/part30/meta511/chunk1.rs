//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 30 (v4rho3sigma_6) CSE chunk 1835/2341 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1835(t1066: f64, t1920: f64, t23346: f64, t23385: f64, t23387: f64, t23389: f64, t25767: f64, t25778: f64, t25785: f64, t25789: f64, t25791: f64, t3026: f64, t3169: f64, t388: f64, t4557: f64, t4660: f64, t4665: f64, t6687: f64, t6771: f64, t6776: f64, t6816: f64, t7554: f64, t7566: f64, t7600: f64, t7625: f64) -> f64 {
    let t25794 = -t4660 * t6816 + 0.82246703342411321825e-2_f64 * t1920 * t25767 + 2.0_f64 * t6771 * t4665 + 2.0_f64 * t3169 * t7600 + 0.21932454224643019153e-1_f64 * t23346 * t7566 + 2.0_f64 * t4557 * t6776 - t25778 * t1066 - 0.27415567780803773942e-2_f64 * t23385 - 0.27415567780803773942e-2_f64 * t23387 - 0.73108180748810063845e-2_f64 * t23346 * t7554 + 0.82246703342411321825e-2_f64 * t6687 * t25785 - 0.73108180748810063845e-2_f64 * t23389 + t25789 * t388 + t25791 * t388 - t3026 * t7625;
    t25794
}
