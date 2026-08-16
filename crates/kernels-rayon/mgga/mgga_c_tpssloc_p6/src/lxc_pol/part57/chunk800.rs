//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 57 (v4rho2sigma2_13) CSE chunk 800/1049 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part57_v4rho2sigma2_13_chunk800(t1409: f64, t1634: f64, t23330: f64, t23329: f64, t25442: f64, t7553: f64, t1955: f64, t5943: f64, t3174: f64, t1052: f64, t17575: f64, t17588: f64, t18074: f64, t1956: f64, t23327: f64, t23359: f64, t25807: f64, t25824: f64, t28594: f64, t28679: f64, t28681: f64, t28684: f64, t28691: f64, t28697: f64, t388: f64, t4557: f64, t5920: f64, t5944: f64, t6687: f64, t6771: f64, t7600: f64, t7625: f64) -> f64 {
    let t28701 = t23330 * t1409 * t1634;
    let t28702 = t23329 * t28701;
    let t28705 = t25442 * t7553;
    let t28712 = t1955 * t5943;
    let t28713 = t3174 * t28712;
    let t28718 = -t6771 * t5944 + t28594 * t388 - t1052 * t28679 - t23359 - 0.16449340668482264365e-1_f64 * t6687 * t28681 - 0.16449340668482264365e-1_f64 * t6687 * t28684 - 2.0_f64 * t17588 * t1956 - 2.0_f64 * t4557 * t7625 - 0.82246703342411321825e-2_f64 * t6687 * t28691 + 4.0_f64 * t4557 * t7600 - 6.0_f64 * t1052 * t28697 - 0.54831135561607547884e-2_f64 * t23327 * t28702 - 0.54831135561607547884e-2_f64 * t23327 * t28705 + 0.54831135561607547884e-2_f64 * t25807 - t17575 * t1956 + 2.0_f64 * t6771 * t5920 + 2.0_f64 * t1052 * t28713 - 0.54831135561607547884e-2_f64 * t25824 - t18074 * t1956;
    t28718
}
