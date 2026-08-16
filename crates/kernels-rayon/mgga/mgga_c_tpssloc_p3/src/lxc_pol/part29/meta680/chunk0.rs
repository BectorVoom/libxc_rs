//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 29 (v4rho3sigma_5) CSE chunk 2286/2357 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2286(t24574: f64, t27383: f64, t7288: f64, t94490: f64, t11613: f64, t1190: f64, t15820: f64, t24634: f64, t24880: f64, t24883: f64, t24887: f64, t27406: f64, t27426: f64, t27721: f64, t27742: f64, t27747: f64, t3481: f64, t3487: f64, t3593: f64, t498: f64, t5089: f64, t7283: f64, t7356: f64, t8054: f64, t8061: f64, t86390: f64) -> f64 {
    let t94628 = 0.54831135561607547884e-2_f64 * t24574 * t27383;
    let t94631 = t94490 * t7288;
    let t94637 = 4.0_f64 * t11613 * t8061 - 0.27415567780803773942e-2_f64 * t7283 * t27426 * t24883 - 0.54831135561607547884e-2_f64 * t7283 * t27426 * t24887 + 4.0_f64 * t3593 * t27747 + 0.27415567780803773942e-2_f64 * t86390 - 2.0_f64 * t3487 * t27742 + t3481 * t8054 * t498 + 2.0_f64 * t1190 * t27721 * t498 + t94628 + 0.14621636149762012769e-1_f64 * t27406 * t24634 + 0.48738787165873375896e-2_f64 * t94631 + 4.0_f64 * t15820 * t7356 - 2.0_f64 * t24880 * t5089;
    t94637
}
