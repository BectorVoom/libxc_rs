//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 27 (v4rho3sigma_3) CSE chunk 2229/2372 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2229(t1598: f64, t3008: f64, t23384: f64, t25407: f64, t25513: f64, t82431: f64, t25726: f64, t14165: f64, t14626: f64, t23327: f64, t23601: f64, t23603: f64, t23604: f64, t23613: f64, t23670: f64, t23677: f64, t23678: f64, t25471: f64, t25475: f64, t25503: f64, t25510: f64, t25545: f64, t25721: f64, t7603: f64, t82402: f64, t82750: f64) -> (f64, f64, f64) {
    let t88941 = t1598 * t3008;
    let t88954 = 0.54831135561607547884e-2_f64 * t23384 * t25407;
    let t88992 = 0.36554090374405031922e-2_f64 * t82431 * t25513;
    let t88998 = 0.18277045187202515961e-2_f64 * t82431 * t25726;
    let t89001 = -0.27415567780803773942e-2_f64 * t23327 * t82750 * t7603 + 0.16449340668482264365e-1_f64 * t23601 * t23677 * t14626 * t23678 - 0.54831135561607547884e-2_f64 * t23327 * t23613 * t25475 + 0.14621636149762012769e-1_f64 * t82402 * t25471 - 0.82246703342411321825e-2_f64 * t23601 * t23603 * t14626 * t23604 + 0.14621636149762012769e-1_f64 * t82402 * t25726 - 0.43864908449286038306e-1_f64 * t23670 * t25545 + 0.29243272299524025538e-1_f64 * t82402 * t25513 - t88992 + 0.16449340668482264365e-1_f64 * t23327 * t25510 * t25721 * t14165 - t88998 - 0.43864908449286038306e-1_f64 * t23670 * t25503;
    (t88941, t88954, t89001)
}
