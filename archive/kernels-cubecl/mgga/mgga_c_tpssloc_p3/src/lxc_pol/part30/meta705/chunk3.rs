//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 30 (v4rho3sigma_6) CSE chunk 2311/2341 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2311<F: Float>(t100236: F, t1003: F, t1022: F, t17187: F, t18086: F, t23346: F, t23633: F, t23635: F, t25500: F, t28634: F, t28653: F, t28660: F, t353: F, t383: F, t4542: F, t4669: F, t5398: F, t6687: F, t6784: F, t6785: F, t6800: F, t6811: F, t7614: F, t82668: F, t83233: F, t89329: F, t99859: F) -> F {
    let t100314 = F::cast_from(0.27415567780803773942e-2_f64) * t23633 * t23635 * t5398 * t1022 * t6800 + t89329 - F::cast_from(0.16449340668482264365e-1_f64) * t6687 * t4542 * t7614 + F::cast_from(2.0_f64) * t4669 * t25500 + t1003 * t28634 - F::cast_from(0.54831135561607547884e-2_f64) * t23633 * t83233 * t100236 + t353 * t383 * t99859 + F::cast_from(0.27415567780803773942e-2_f64) * t6687 * t6784 * t6785 * t17187 - F::cast_from(0.14621636149762012769e-1_f64) * t82668 * t28653 + F::cast_from(0.21932454224643019153e-1_f64) * t23346 * t28660 + t18086 * t6811;
    t100314
}
