//! MGGA_C_REVTPSS lxc pol — lxc_pol part 36 (v4rho3sigma_11) CSE chunk 1355/1378 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part36_v4rho3sigma_11_chunk1355<F: Float>(t104963: F, t112433: F, t112435: F, t112437: F, t112452: F, t112461: F, t112465: F, t112468: F, t24236: F, t24636: F, t24794: F, t24798: F, t26867: F, t29047: F, t29054: F, t29089: F, t6653: F, t7613: F, t97272: F) -> F {
    let t116234 = t29047 * t29054 * t24236 / F::cast_from(72.0_f64) + t112433 / F::cast_from(54.0_f64) - t112435 / F::cast_from(288.0_f64) - t112437 / F::cast_from(144.0_f64) - F::cast_from(0.17149607247227894789e-2_f64) * t26867 * t24798 - F::cast_from(0.85748036236139473944e-3_f64) * t26867 * t24794 - F::cast_from(0.85748036236139473944e-3_f64) * t112452 - t29089 * t6653 / F::cast_from(27.0_f64) + t104963 / F::cast_from(54.0_f64) + t97272 + F::cast_from(0.85748036236139473944e-3_f64) * t112461 - F::cast_from(0.91464571985215438873e-2_f64) * t112465 + F::cast_from(0.11433071498151929859e-2_f64) * t112468 - F::cast_from(0.42874018118069736972e-3_f64) * t7613 * t24636;
    t116234
}
