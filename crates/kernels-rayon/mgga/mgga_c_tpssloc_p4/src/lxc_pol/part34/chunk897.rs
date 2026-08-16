//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 34 (v4rho3sigma_10) CSE chunk 897/1250 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part34_v4rho3sigma_10_chunk897(t13598: f64, t13642: f64, t17149: f64, t17165: f64, t17175: f64, t17286: f64, t17288: f64, t17290: f64, t21161: f64, t21168: f64, t21181: f64, t21183: f64, t21186: f64, t21188: f64) -> f64 {
    let t21237 = -0.27595e0_f64 * t13642 + 0.49671e0_f64 * t21161 - 0.40256666666666666668e0_f64 * t13598 + 0.20128333333333333333e0_f64 * t17149 - 0.60385000000000000001e0_f64 * t17165 + 0.30192500000000000001e0_f64 * t17175 - 0.82785e-1_f64 * t21168 + 0.258925e1_f64 * t21181 + 0.16504875e0_f64 * t21183 - 0.412621875e-1_f64 * t21186 + 0.19419375e1_f64 * t21188 + 0.5519e-1_f64 * t17286 - 0.33114e0_f64 * t17288 + 0.16557e0_f64 * t17290;
    t21237
}
