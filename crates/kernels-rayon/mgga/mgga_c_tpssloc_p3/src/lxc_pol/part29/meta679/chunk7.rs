//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 29 (v4rho3sigma_5) CSE chunk 2283/2357 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2283(t2122: f64, t94319: f64, t8034: f64, t8003: f64, t85660: f64, t1186: f64, t11928: f64, t15786: f64, t24582: f64, t24589: f64, t24604: f64, t24633: f64, t27388: f64, t27396: f64, t27830: f64, t3487: f64, t3600: f64, t5055: f64, t7283: f64, t7300: f64, t7301: f64, t8014: f64, t8061: f64, t85707: f64, t85739: f64, t85741: f64, t85766: f64) -> f64 {
    let t94503 = t2122 * t94319;
    let t94514 = t8034 * t2122;
    let t94525 = t85660 * t8003;
    let t94530 = -0.54831135561607547884e-2_f64 * t7283 * t24633 * t27388 - 0.54831135561607547884e-2_f64 * t85739 + 0.16449340668482264365e-1_f64 * t7283 * t1186 * t94503 - 0.18277045187202515961e-2_f64 * t85741 + 4.0_f64 * t3487 * t27396 + 4.0_f64 * t5055 * t24582 + 2.0_f64 * t11928 * t8061 - 0.54831135561607547884e-2_f64 * t24589 * t94514 * t24604 - 0.82246703342411321825e-2_f64 * t7283 * t7300 * t7301 * t15786 + 2.0_f64 * t27830 * t3600 - 0.54831135561607547884e-2_f64 * t85766 + 0.60923483957341719871e-3_f64 * t94525 - 0.82246703342411321825e-2_f64 * t7283 * t85707 * t8014;
    t94530
}
