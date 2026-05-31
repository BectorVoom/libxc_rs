//! MGGA_C_REVTPSS lxc pol — lxc_pol part 56 (v4rho2sigma2_11) CSE chunk 1169/1203 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part56_v4rho2sigma2_11_chunk1169<F: Float>(t116: F, t34873: F, t13272: F, t33362: F, t124456: F, t124463: F, t124480: F, t124483: F, t125261: F, t125269: F, t125280: F, t125283: F, t125290: F, t125298: F, t125305: F, t125309: F, t125313: F, t125314: F, t125337: F, t1470: F, t32145: F, t32798: F, t32802: F, t33359: F, t33363: F, t33612: F, t33617: F, t36: F, t606: F, t7574: F, t8142: F, t8442: F, t8621: F, t8912: F, t8913: F) -> (F, F) {
    let t131234 = t34873 * t116;
    let t131256 = t13272 * t33362;
    let t131276 = -F::cast_from(5.0_f64) / F::cast_from(18.0_f64) * t124463 * t125269 - F::cast_from(5.0_f64) / F::cast_from(18.0_f64) * t32802 * t8442 * t8142 * t36 * t606 - F::cast_from(5.0_f64) / F::cast_from(18.0_f64) * t124463 * t125280 + F::cast_from(5.0_f64) / F::cast_from(6.0_f64) * t124456 * t125337 + F::cast_from(5.0_f64) / F::cast_from(6.0_f64) * t124456 * t125261 - F::cast_from(5.0_f64) / F::cast_from(18.0_f64) * t32802 * t8442 * t1470 * t7574 + F::cast_from(5.0_f64) / F::cast_from(144.0_f64) * t125283 * t8913 - F::cast_from(5.0_f64) / F::cast_from(36.0_f64) * t131256 * t32145 + F::cast_from(35.0_f64) / F::cast_from(24.0_f64) * t124483 * t125290 - F::cast_from(5.0_f64) / F::cast_from(12.0_f64) * t32798 * t8621 * t33612 * t7574 - F::cast_from(5.0_f64) / F::cast_from(12.0_f64) * t33359 * t125298 - F::cast_from(5.0_f64) / F::cast_from(36.0_f64) * t124480 * t33617 - F::cast_from(5.0_f64) / F::cast_from(36.0_f64) * t33363 * t125305 - F::cast_from(5.0_f64) / F::cast_from(36.0_f64) * t33363 * t125309 + F::cast_from(5.0_f64) / F::cast_from(18.0_f64) * t125313 * t8912 * t125314;
    (t131234, t131276)
}
