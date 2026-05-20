//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 3174/3317 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3174<F: Float>(t57242: F, t57251: F, t57257: F, t57271: F, t57274: F, t57331: F, t57548: F, t59330: F, t70511: F, t70521: F, t70523: F, t70542: F, t77513: F) -> F {
    let t83361 = -F::cast_from(0.28582678745379824648e-3_f64) * t70511 - F::cast_from(0.42874018118069736972e-3_f64) * t70521 + F::cast_from(0.85748036236139473944e-3_f64) * t70523 - t57242 + t57251 + t57257 - F::cast_from(0.42874018118069736972e-3_f64) * t70542 - t57271 + t57274 + F::cast_from(0.95275595817932748825e-4_f64) * t57331 - t57548 * t59330 * t77513 / F::new(16.0);
    t83361
}
