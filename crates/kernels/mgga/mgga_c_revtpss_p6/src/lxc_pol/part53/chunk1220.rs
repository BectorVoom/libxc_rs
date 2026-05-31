//! MGGA_C_REVTPSS lxc pol — lxc_pol part 53 (v4rho2sigma2_8) CSE chunk 1220/1244 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part53_v4rho2sigma2_8_chunk1220<F: Float>(t34382: F, t4254: F, t1936: F, t29337: F, t651: F, t32822: F, t7937: F, t28177: F, t8764: F, t34399: F, t7239: F, t125512: F, t125514: F, t125515: F, t125517: F, t125521: F, t125522: F, t2007: F, t29422: F, t7221: F, t8152: F) -> F {
    let t129332 = t4254 * t34382;
    let t129335 = t651 * t29337 * t1936;
    let t129339 = t32822 * t7937;
    let t129342 = t8764 * t28177;
    let t129344 = t34399 * t7239;
    let t129346 = -t2007 * t29422 - t7221 * t8152 + t125512 - t125514 - F::cast_from(2.0_f64) * t125515 - F::cast_from(2.0_f64) * t125517 - t125521 - t125522 - F::cast_from(2.0_f64) * t129332 - F::cast_from(2.0_f64) * t129335 - t129339 + F::cast_from(3.0_f64) * t129342 + F::cast_from(3.0_f64) * t129344;
    t129346
}
