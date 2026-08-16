//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 29 (v4rho3sigma_5) CSE chunk 2328/2357 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2328(t24574: f64, t27574: f64, t24844: f64, t7999: f64, t1244: f64, t1246: f64, t15015: f64, t15027: f64, t1729: f64, t24792: f64, t24863: f64, t27470: f64, t27724: f64, t3471: f64, t3493: f64, t3624: f64, t470: f64, t493: f64, t5079: f64, t7283: f64, t7373: f64, t7375: f64, t7376: f64, t8054: f64, t8077: f64, t86020: f64, t95707: f64) -> f64 {
    let t95714 = 0.54831135561607547884e-2_f64 * t24574 * t27574;
    let t95722 = 0.14621636149762012769e-1_f64 * t7999 * t24844;
    let t95723 = t1729 * t24792 - 2.0_f64 * t3624 * t27724 * t5079 - 2.0_f64 * t3624 * t27470 * t5079 + 0.82246703342411321825e-2_f64 * t7373 * t7375 * t15015 * t7376 + t470 * t493 * t95707 + t1244 * t8054 * t3493 * t1246 - t95714 - 0.82246703342411321825e-2_f64 * t7283 * t3471 * t8077 - 0.54831135561607547884e-2_f64 * t86020 + 2.0_f64 * t15027 * t24863 - t95722;
    t95723
}
