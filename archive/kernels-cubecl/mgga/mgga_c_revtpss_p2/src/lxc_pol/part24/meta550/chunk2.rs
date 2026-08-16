//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1628/1850 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1628<F: Float>(t14546: F, t18677: F, t39649: F, t39652: F, t51390: F, t51403: F, t51408: F, t62684: F, t62716: F, t62723: F, t76237: F, t76242: F, t76255: F) -> F {
    let t87824 = F::cast_from(0.39029762157531132076e-1_f64) * t76237 + t39649 - t39652 - F::cast_from(0.23707617058567841754e2_f64) * t14546 * t18677 * t76242 - F::cast_from(0.7805952431506226415e-2_f64) * t62684 + F::cast_from(0.1040793657534163522e-1_f64) * t51390 - F::cast_from(0.11708928647259339623e0_f64) * t76255 - F::cast_from(0.68293547082294194357e-1_f64) * t51403 - F::cast_from(0.12142592671231907757e0_f64) * t51408 + F::cast_from(0.69394917116090352835e-2_f64) * t62716 - F::cast_from(0.69394917116090352835e-2_f64) * t62723;
    t87824
}
