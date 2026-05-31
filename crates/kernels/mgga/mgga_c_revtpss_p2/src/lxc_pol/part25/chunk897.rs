//! MGGA_C_REVTPSS lxc pol — lxc_pol part 25 (v4rho3sigma_0) CSE chunk 897/1360 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part25_v4rho3sigma_0_chunk897<F: Float>(t10605: F, t2612: F, t2523: F, t2626: F, t760: F, t9425: F, t2609: F, t606: F, t706: F, t10592: F, t10594: F, t10596: F, t10598: F, t10602: F, t10604: F, t9542: F) -> (F, F, F, F, F) {
    let t10607 = F::cast_from(36.0_f64) * t10605 * t2612;
    let t10608 = t2523 * t2626;
    let t10609 = F::cast_from(0.35089341735807877242e1_f64) * t10608;
    let t10611 = F::cast_from(0.35089341735807877242e1_f64) * t760 * t9425;
    let t10612 = t2609 * t606;
    let t10613 = t706 * t10612;
    let t10614 = F::cast_from(12.0_f64) * t10613;
    let t10615 = t10592 - t10594 - t10596 - t10598 + t10602 - t10604 + t9542 + t10607 + t10609 - t10611 + t10614;
    (t10607, t10609, t10611, t10614, t10615)
}
