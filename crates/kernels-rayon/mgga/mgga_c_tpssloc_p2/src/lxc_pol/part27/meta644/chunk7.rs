//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 27 (v4rho3sigma_3) CSE chunk 2205/2372 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2205(t4616: f64, t6764: f64, t23544: f64, t4571: f64, t23482: f64, t25682: f64, t25588: f64, t344: f64, t6740: f64, t1046: f64, t14093: f64, t14174: f64, t14230: f64, t23419: f64, t23483: f64, t25679: f64, t6747: f64, t6765: f64, t7583: f64, t82883: f64, t82885: f64, t82893: f64, t82897: f64, t83114: f64) -> f64 {
    let t88277 = t4616 * t6764;
    let t88281 = t23544 * t4571 / 1728.0_f64;
    let t88286 = t23482 * t25682;
    let t88290 = t6740 * t25588 * t344;
    let t88303 = t88277 * t1046 / 1152.0_f64 + t88281 - 0.16149102437656156342e-2_f64 * t83114 * t7583 - 0.16149102437656156342e-2_f64 * t23483 * t25679 - 0.16149102437656156342e-2_f64 * t88286 * t6747 + 0.20186378047070195428e-3_f64 * t88290 * t6747 + t82883 / 2304.0_f64 + t82885 / 648.0_f64 + 0.20186378047070195428e-3_f64 * t82893 - 0.10093189023535097714e-3_f64 * t82897 - t23419 * t14230 / 576.0_f64 - 5.0_f64 / 1152.0_f64 * t6765 * t14174 + t6765 * t14093 / 2304.0_f64;
    t88303
}
