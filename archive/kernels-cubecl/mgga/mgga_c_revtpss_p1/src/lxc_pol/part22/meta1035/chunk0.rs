//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3620/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3620<F: Float>(t68284: F, t68338: F, t68379: F, t68466: F, t68501: F, t68526: F, t68564: F, t68595: F, t1179: F, t1188: F, t1196: F, t20397: F, t3531: F) -> (F, F, F) {
    let t68598 = t68284 + t68338 + t68379 + t68466 + t68501 + t68526 + t68564 + t68595;
    let t68602 = F::cast_from(0.5848223622634646207e0_f64) * t1196 * t1179 * t68598 * t1188;
    let t68604 = F::cast_from(0.69263436422725855036e2_f64) * t3531 * t20397;
    (t68598, t68602, t68604)
}
