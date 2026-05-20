//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 3200/3317 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3200<F: Float>(t21213: F, t5369: F, t59186: F, t71550: F, t71552: F, t71571: F, t71582: F, t71598: F, t71630: F, t71687: F, t71710: F, t71718: F) -> F {
    let t84049 = F::cast_from(0.85748036236139473944e-3_f64) * t71550 + F::cast_from(0.85748036236139473944e-3_f64) * t71552 + t71571 / F::new(36.0) + t71582 / F::new(108.0) - F::new(11.0) / F::new(108.0) * t21213 * t5369 - F::cast_from(0.57165357490759649295e-3_f64) * t71598 + t59186 - F::cast_from(0.85748036236139473944e-3_f64) * t71630 + F::cast_from(0.57165357490759649295e-3_f64) * t71687 - F::cast_from(0.45732285992607719436e-2_f64) * t71710 - t71718 / F::new(81.0);
    t84049
}
