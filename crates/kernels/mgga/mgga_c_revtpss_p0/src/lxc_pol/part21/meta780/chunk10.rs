//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2791/3259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2791<F: Float>(t51268: F, t14939: F, t213: F, t4470: F, t786: F, t867: F, t2467: F, t14567: F, t2453: F, t10538: F, t14662: F, t251: F) -> (F, F, F, F, F) {
    let t51269 = F::cast_from(0.34697458558045176417e-2_f64) * t51268;
    let t51272 = t213 * t14939;
    let t51276 = t786 * t4470 * t867;
    let t51277 = t51276 * t2467;
    let t51297 = t2453 * t14567;
    let t51298 = t51297 * t10538;
    let t51299 = F::cast_from(0.34697458558045176417e-2_f64) * t51298;
    let t51306 = t251 * t14662;
    (t51269, t51272, t51277, t51299, t51306)
}
