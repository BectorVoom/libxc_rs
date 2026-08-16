//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 29 (v4rho3sigma_5) CSE chunk 2282/2357 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2282(t7303: f64, t94490: f64, t7291: f64, t11605: f64, t1186: f64, t1251: f64, t1761: f64, t2155: f64, t24589: f64, t24601: f64, t24602: f64, t27761: f64, t27766: f64, t27784: f64, t3487: f64, t3966: f64, t5059: f64, t51937: f64, t7283: f64, t7391: f64, t8002: f64, t85711: f64, t85717: f64, t85724: f64, t85733: f64, t94475: f64, t94476: f64) -> f64 {
    let t94492 = 0.14621636149762012769e-1_f64 * t94490 * t7303;
    let t94494 = 0.14621636149762012769e-1_f64 * t94490 * t7291;
    let t94498 = 4.0_f64 * t3487 * t27761 - 0.27415567780803773942e-2_f64 * t85711 - t51937 * t2155 - 0.16449340668482264365e-1_f64 * t7283 * t1186 * t27766 - t94475 + 0.18277045187202515961e-2_f64 * t94476 + 0.54831135561607547884e-2_f64 * t24589 * t24601 * t24602 * t3966 * t1251 + 0.54831135561607547884e-2_f64 * t24589 * t85724 * t8002 - 12.0_f64 * t27784 * t11605 * t7391 * t5059 + t94492 + t94494 + 0.54831135561607547884e-2_f64 * t85733 - 2.0_f64 * t85717 * t1761;
    t94498
}
