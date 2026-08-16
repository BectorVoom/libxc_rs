//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 30 (v4rho3sigma_6) CSE chunk 2298/2341 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2298<F: Float>(t1920: F, t28474: F, t968: F, t14529: F, t14555: F, t1599: F, t17187: F, t1956: F, t23372: F, t25766: F, t28485: F, t3026: F, t4542: F, t5920: F, t61061: F, t6687: F, t6689: F, t6690: F, t7561: F, t7600: F, t7625: F, t88182: F, t89561: F, t89583: F, t89597: F) -> F {
    let t99877 = t1920 * t968 * t28474;
    let t99894 = t89561 - t61061 * t1956 - F::cast_from(0.16449340668482264365e-1_f64) * t6687 * t4542 * t7561 - F::cast_from(0.16449340668482264365e-1_f64) * t6687 * t1599 * t25766 + F::cast_from(0.27415567780803773942e-2_f64) * t99877 + t89583 + F::cast_from(2.0_f64) * t23372 * t5920 + F::cast_from(0.16449340668482264365e-1_f64) * t6687 * t1599 * t88182 + F::cast_from(0.27415567780803773942e-2_f64) * t6687 * t6689 * t6690 * t17187 + F::cast_from(4.0_f64) * t3026 * t28485 - F::cast_from(2.0_f64) * t14555 * t7625 - t89597 + F::cast_from(4.0_f64) * t14529 * t7600;
    t99894
}
