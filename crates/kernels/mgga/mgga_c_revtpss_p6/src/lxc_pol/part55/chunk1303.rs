//! MGGA_C_REVTPSS lxc pol — lxc_pol part 55 (v4rho2sigma2_10) CSE chunk 1303/1306 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part55_v4rho2sigma2_10_chunk1303<F: Float>(t129099: F, t129103: F, t129107: F, t129109: F, t129111: F, t1918: F, t2115: F, t2170: F, t28975: F, t28981: F, t29480: F, t33328: F, t34011: F, t34014: F, t5802: F, t8616: F, t8905: F) -> F {
    let t131170 = F::new(3.0) * t1918 * t33328 + F::new(3.0) * t2115 * t29480 + F::new(6.0) * t2170 * t28975 + F::new(6.0) * t2170 * t28981 + F::new(6.0) * t5802 * t8905 + t129099 + t129103 + t129107 + t129109 + t129111 + t34011 + t34014 + t8616;
    t131170
}
