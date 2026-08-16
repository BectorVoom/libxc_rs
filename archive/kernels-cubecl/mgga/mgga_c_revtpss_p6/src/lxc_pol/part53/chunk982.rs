//! MGGA_C_REVTPSS lxc pol — lxc_pol part 53 (v4rho2sigma2_8) CSE chunk 982/1244 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part53_v4rho2sigma2_8_chunk982<F: Float>(t1518: F, t27060: F, t28212: F, t28214: F, t28216: F, t28218: F, t28221: F, t28223: F, t28225: F, t28227: F, t28229: F, t29422: F, t29427: F, t29432: F, t4292: F, t670: F, t7586: F) -> F {
    let t29437 = F::cast_from(2.0_f64) * t1518 * t27060 + F::cast_from(2.0_f64) * t1518 * t29432 + F::cast_from(2.0_f64) * t29427 * t670 + F::cast_from(2.0_f64) * t4292 * t7586 + t28212 + t28214 + t28216 + t28218 + t28221 + t28223 + t28225 + t28227 + t28229 + t29422;
    t29437
}
