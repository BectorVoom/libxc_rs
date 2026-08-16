//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 30 (v4rho3sigma_6) CSE chunk 2047/2341 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2047(t25425: f64, t82431: f64, t25816: f64, t25443: f64, t1049: f64, t7577: f64, t7557: f64, t82573: f64, t23384: f64, t25785: f64, t25447: f64, t1625: f64, t6733: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t88069 = 0.36554090374405031922e-2_f64 * t82431 * t25425;
    let t88075 = 0.18277045187202515961e-2_f64 * t82431 * t25816;
    let t88083 = 0.18277045187202515961e-2_f64 * t82431 * t25443;
    let t88089 = t7577 * t1049;
    let t88096 = 0.14621636149762012769e-1_f64 * t82573 * t7557;
    let t88100 = 0.54831135561607547884e-2_f64 * t23384 * t25785;
    let t88102 = 0.54831135561607547884e-2_f64 * t23384 * t25447;
    let t88105 = t6733 * t1625;
    (t88069, t88075, t88083, t88089, t88096, t88100, t88102, t88105)
}
