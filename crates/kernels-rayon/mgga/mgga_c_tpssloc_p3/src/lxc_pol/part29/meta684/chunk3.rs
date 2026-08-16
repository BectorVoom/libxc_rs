//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 29 (v4rho3sigma_5) CSE chunk 2329/2357 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2329(t2121: f64, t3427: f64, t8077: f64, t27517: f64, t85639: f64, t24574: f64, t27481: f64, t11888: f64, t11904: f64, t15022: f64, t15247: f64, t24589: f64, t24794: f64, t24798: f64, t24841: f64, t24849: f64, t27516: f64, t27532: f64, t27543: f64, t3565: f64, t3624: f64, t5064: f64, t5072: f64, t7327: f64, t8082: f64, t8085: f64, t86057: f64) -> f64 {
    let t95726 = t2121 * t3427 * t8077;
    let t95747 = 0.18277045187202515961e-2_f64 * t85639 * t27517;
    let t95751 = 0.54831135561607547884e-2_f64 * t24574 * t27481;
    let t95752 = t3565 * t8085 - 0.18277045187202515961e-2_f64 * t95726 - t3624 * t8082 * t15022 - 0.54831135561607547884e-2_f64 * t24849 * t7327 * t5072 * t27532 + 4.0_f64 * t11904 * t27543 + 0.27415567780803773942e-2_f64 * t86057 - 6.0_f64 * t11888 * t8082 * t15247 + 0.27415567780803773942e-2_f64 * t24589 * t27516 * t24794 + 0.54831135561607547884e-2_f64 * t24589 * t27516 * t24798 + t95747 + 2.0_f64 * t5064 * t24841 - t95751;
    t95752
}
