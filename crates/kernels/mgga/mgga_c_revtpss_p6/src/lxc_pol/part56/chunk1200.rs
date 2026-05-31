//! MGGA_C_REVTPSS lxc pol — lxc_pol part 56 (v4rho2sigma2_11) CSE chunk 1200/1203 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part56_v4rho2sigma2_11_chunk1200<F: Float>(t127489: F, t127495: F, t127503: F, t127507: F, t129555: F, t129559: F, t129562: F, t129564: F, t129570: F, t129572: F, t129574: F, t129577: F, t1461: F, t34011: F, t34014: F, t35027: F, t5802: F, t5805: F, t8616: F, t8975: F) -> F {
    let t132167 = F::cast_from(3.0_f64) * t1461 * t35027 + F::cast_from(6.0_f64) * t5802 * t8975 + F::cast_from(3.0_f64) * t5805 * t8975 + t127489 + t127495 + t127503 + t127507 + F::cast_from(6.0_f64) * t129555 + F::cast_from(12.0_f64) * t129559 + F::cast_from(12.0_f64) * t129562 + F::cast_from(6.0_f64) * t129564 + F::cast_from(12.0_f64) * t129570 + F::cast_from(12.0_f64) * t129572 + F::cast_from(12.0_f64) * t129574 + F::cast_from(6.0_f64) * t129577 + t34011 + t34014 + t8616;
    t132167
}
