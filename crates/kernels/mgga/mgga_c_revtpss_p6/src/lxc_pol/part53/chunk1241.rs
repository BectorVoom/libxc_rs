//! MGGA_C_REVTPSS lxc pol — lxc_pol part 53 (v4rho2sigma2_8) CSE chunk 1241/1244 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part53_v4rho2sigma2_8_chunk1241<F: Float>(t2170: F, t28268: F, t28277: F, t28265: F, t28280: F, t127496: F, t127498: F, t127500: F, t127503: F, t127507: F, t1918: F, t32897: F, t34014: F, t8616: F) -> F {
    let t129570 = t2170 * t28268;
    let t129572 = t2170 * t28277;
    let t129574 = t2170 * t28265;
    let t129577 = t2170 * t28280;
    let t129580 = F::cast_from(3.0_f64) * t1918 * t32897 + F::cast_from(3.0_f64) * t127496 + F::cast_from(3.0_f64) * t127498 + F::cast_from(6.0_f64) * t127500 + t127503 + t127507 + F::cast_from(6.0_f64) * t129570 + F::cast_from(6.0_f64) * t129572 + F::cast_from(6.0_f64) * t129574 + F::cast_from(3.0_f64) * t129577 + t34014 + t8616;
    t129580
}
