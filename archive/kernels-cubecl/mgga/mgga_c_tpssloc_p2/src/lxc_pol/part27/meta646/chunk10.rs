//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 27 (v4rho3sigma_3) CSE chunk 2229/2372 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2229<F: Float>(t1598: F, t3008: F, t23384: F, t25407: F, t25513: F, t82431: F, t25726: F, t14165: F, t14626: F, t23327: F, t23601: F, t23603: F, t23604: F, t23613: F, t23670: F, t23677: F, t23678: F, t25471: F, t25475: F, t25503: F, t25510: F, t25545: F, t25721: F, t7603: F, t82402: F, t82750: F) -> (F, F, F) {
    let t88941 = t1598 * t3008;
    let t88954 = F::cast_from(0.54831135561607547884e-2_f64) * t23384 * t25407;
    let t88992 = F::cast_from(0.36554090374405031922e-2_f64) * t82431 * t25513;
    let t88998 = F::cast_from(0.18277045187202515961e-2_f64) * t82431 * t25726;
    let t89001 = -F::cast_from(0.27415567780803773942e-2_f64) * t23327 * t82750 * t7603 + F::cast_from(0.16449340668482264365e-1_f64) * t23601 * t23677 * t14626 * t23678 - F::cast_from(0.54831135561607547884e-2_f64) * t23327 * t23613 * t25475 + F::cast_from(0.14621636149762012769e-1_f64) * t82402 * t25471 - F::cast_from(0.82246703342411321825e-2_f64) * t23601 * t23603 * t14626 * t23604 + F::cast_from(0.14621636149762012769e-1_f64) * t82402 * t25726 - F::cast_from(0.43864908449286038306e-1_f64) * t23670 * t25545 + F::cast_from(0.29243272299524025538e-1_f64) * t82402 * t25513 - t88992 + F::cast_from(0.16449340668482264365e-1_f64) * t23327 * t25510 * t25721 * t14165 - t88998 - F::cast_from(0.43864908449286038306e-1_f64) * t23670 * t25503;
    (t88941, t88954, t89001)
}
