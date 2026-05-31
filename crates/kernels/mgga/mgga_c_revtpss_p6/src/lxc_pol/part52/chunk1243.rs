//! MGGA_C_REVTPSS lxc pol — lxc_pol part 52 (v4rho2sigma2_7) CSE chunk 1243/1292 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part52_v4rho2sigma2_7_chunk1243<F: Float>(t119457: F, t128409: F, t640: F, t121617: F, t121625: F, t121630: F, t121646: F, t121647: F, t121660: F, t125238: F, t125265: F, t125274: F, t128368: F, t128371: F, t128374: F, t128377: F, t128382: F, t128385: F, t128394: F, t128399: F, t128403: F, t128411: F, t32151: F, t32581: F, t32590: F, t32593: F, t33621: F, t34169: F, t34177: F, t45972: F, t60221: F, t8619: F, t8620: F, t8623: F) -> F {
    let t128415 = t119457 * t128409 * t640;
    let t128422 = -F::cast_from(5.0_f64) / F::cast_from(3.0_f64) * t121660 * t128368 - F::cast_from(5.0_f64) / F::cast_from(3.0_f64) * t121660 * t128371 + F::cast_from(5.0_f64) / F::cast_from(9.0_f64) * t32590 * t128374 + F::cast_from(5.0_f64) / F::cast_from(9.0_f64) * t32590 * t128377 - F::cast_from(5.0_f64) / F::cast_from(72.0_f64) * t8620 * t125238 + F::cast_from(5.0_f64) / F::cast_from(27.0_f64) * t128382 + F::cast_from(5.0_f64) / F::cast_from(27.0_f64) * t128385 - F::cast_from(5.0_f64) / F::cast_from(72.0_f64) * t60221 * t8619 * t8623 - F::cast_from(5.0_f64) / F::cast_from(72.0_f64) * t34169 * t32151 - F::cast_from(5.0_f64) / F::cast_from(72.0_f64) * t32581 * t33621 + F::cast_from(5.0_f64) / F::cast_from(18.0_f64) * t128394 * t32593 + F::cast_from(5.0_f64) / F::cast_from(27.0_f64) * t121617 - F::cast_from(20.0_f64) / F::cast_from(27.0_f64) * t121630 - F::cast_from(20.0_f64) / F::cast_from(27.0_f64) * t128399 + F::cast_from(5.0_f64) / F::cast_from(6.0_f64) * t121647 * t128403 + F::cast_from(5.0_f64) / F::cast_from(18.0_f64) * t32590 * t125274 - F::cast_from(35.0_f64) / F::cast_from(12.0_f64) * t45972 * t121646 * t128411 + F::cast_from(5.0_f64) / F::cast_from(6.0_f64) * t121647 * t128415 + F::cast_from(5.0_f64) / F::cast_from(18.0_f64) * t121625 * t34177 + F::cast_from(5.0_f64) / F::cast_from(18.0_f64) * t32590 * t125265;
    t128422
}
