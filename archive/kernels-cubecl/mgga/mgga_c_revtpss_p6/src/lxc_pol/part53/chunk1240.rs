//! MGGA_C_REVTPSS lxc pol — lxc_pol part 53 (v4rho2sigma2_8) CSE chunk 1240/1244 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part53_v4rho2sigma2_8_chunk1240<F: Float>(t7696: F, t7953: F, t7950: F, t2170: F, t28271: F, t2042: F, t29480: F, t127489: F, t127490: F, t127492: F, t127495: F, t32373: F, t34011: F, t5805: F, t8771: F) -> F {
    let t129555 = t7696 * t7953;
    let t129559 = t7696 * t7950;
    let t129562 = t2170 * t28271;
    let t129564 = t29480 * t2042;
    let t129566 = F::cast_from(3.0_f64) * t5805 * t8771 + t127489 + F::cast_from(6.0_f64) * t127490 + F::cast_from(3.0_f64) * t127492 + t127495 + F::cast_from(3.0_f64) * t129555 + F::cast_from(6.0_f64) * t129559 + F::cast_from(6.0_f64) * t129562 + F::cast_from(3.0_f64) * t129564 + t32373 + t34011;
    t129566
}
