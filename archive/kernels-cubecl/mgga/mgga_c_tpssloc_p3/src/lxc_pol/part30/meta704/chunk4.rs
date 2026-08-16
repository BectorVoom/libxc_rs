//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 30 (v4rho3sigma_6) CSE chunk 2301/2341 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2301<F: Float>(t28648: F, t82431: F, t28667: F, t82736: F, t23665: F, t28626: F, t18080: F, t18161: F, t23327: F, t23601: F, t23670: F, t23677: F, t23678: F, t25470: F, t25717: F, t6797: F, t6799: F, t6800: F, t82402: F, t82534: F, t88992: F, t88998: F) -> F {
    let t99960 = t82431 * t28648;
    let t99966 = t82736 * t28667;
    let t99977 = t23665 * t28626;
    let t99983 = -F::cast_from(0.18277045187202515961e-2_f64) * t99960 + F::cast_from(0.14621636149762012769e-1_f64) * t82402 * t28648 - F::cast_from(0.43864908449286038307e-1_f64) * t82534 * t28667 + F::cast_from(0.54831135561607547883e-2_f64) * t99966 - t88992 - F::cast_from(0.54831135561607547884e-2_f64) * t23327 * t25470 * t25717 + F::cast_from(0.82246703342411321825e-2_f64) * t6797 * t6799 * t18161 * t6800 - t88998 - F::cast_from(0.43864908449286038307e-1_f64) * t23670 * t28626 + F::cast_from(0.54831135561607547883e-2_f64) * t99977 + F::cast_from(0.16449340668482264365e-1_f64) * t23601 * t23677 * t18080 * t23678;
    t99983
}
