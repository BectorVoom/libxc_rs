//! MGGA_C_REVTPSS lxc pol — lxc_pol part 53 (v4rho2sigma2_8) CSE chunk 1222/1244 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part53_v4rho2sigma2_8_chunk1222<F: Float>(t125525: F, t125531: F, t125532: F, t125536: F, t129354: F, t129357: F, t129366: F, t129371: F, t1518: F, t2322: F, t32791: F, t34394: F, t34431: F, t4254: F, t4292: F, t651: F, t670: F, t8756: F) -> F {
    let t129372 = -F::cast_from(2.0_f64) * t1518 * t32791 * t651 - F::cast_from(2.0_f64) * t34394 * t651 * t670 - F::cast_from(2.0_f64) * t4292 * t651 * t8756 - F::cast_from(2.0_f64) * t2322 * t34431 - F::cast_from(2.0_f64) * t34431 * t4254 - t125525 - t125531 - F::cast_from(3.0_f64) * t125532 + t125536 + F::cast_from(2.0_f64) * t129354 - F::cast_from(2.0_f64) * t129357 - F::cast_from(3.0_f64) * t129366 + t129371;
    t129372
}
