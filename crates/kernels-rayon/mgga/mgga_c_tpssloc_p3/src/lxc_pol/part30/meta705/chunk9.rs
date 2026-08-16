//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 30 (v4rho3sigma_6) CSE chunk 2317/2341 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2317(t100025: f64, t100068: f64, t100103: f64, t100147: f64, t100176: f64, t100195: f64, t100225: f64, t100253: f64, t100287: f64, t100314: f64, t100341: f64, t100377: f64, t100396: f64, t100430: f64, t100459: f64, t1052: f64, t1055: f64, t1603: f64, t17875: f64, t18070: f64, t18074: f64, t18166: f64, t1945: f64, t23581: f64, t25705: f64, t25755: f64, t25757: f64, t28499: f64, t28679: f64, t3169: f64, t388: f64, t4694: f64, t5838: f64, t5848: f64, t6687: f64, t6699: f64, t6768: f64, t6771: f64, t6816: f64, t83459: f64, t88851: f64, t89662: f64, t89672: f64, t99983: f64) -> f64 {
    let t100489 = -t1052 * t1055 * (t99983 + t100025 + t100068 + t100103 + t100147 + t100176 + t100195 + t100225 + t100253 + t100287 + t100314 + t100341 + t100377 + t100396 + t100430 + t100459) - t89662 - t6771 * t18166 + 0.36554090374405031923e-2_f64 * t89672 + t17875 * t1945 * t388 + t5848 * t6768 * t388 - 0.82246703342411321825e-2_f64 * t6687 * t5838 * t6699 - 0.54831135561607547884e-2_f64 * t6687 * t23581 * t28499 + 2.0_f64 * t1603 * t25705 * t388 - t3169 * t28679 + 24.0_f64 * t25757 * t88851 * t18070 + 0.18277045187202515961e-2_f64 * t83459 - 2.0_f64 * t25755 * t4694 - t18074 * t6816;
    t100489
}
