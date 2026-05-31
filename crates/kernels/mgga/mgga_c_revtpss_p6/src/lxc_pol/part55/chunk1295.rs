//! MGGA_C_REVTPSS lxc pol — lxc_pol part 55 (v4rho2sigma2_10) CSE chunk 1295/1306 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part55_v4rho2sigma2_10_chunk1295<F: Float>(t128970: F, t128974: F, t128975: F, t128977: F, t128979: F, t128981: F, t128983: F, t129353: F, t2052: F, t28287: F, t28927: F, t29337: F, t29427: F, t29459: F, t32822: F, t7357: F, t7359: F, t7374: F, t8079: F, t8233: F, t8764: F) -> F {
    let t131092 = F::cast_from(2.0_f64) * t129353 * t28287 - t2052 * t29337 + t28927 * t8764 - F::cast_from(2.0_f64) * t29427 * t7374 - F::cast_from(2.0_f64) * t29459 * t7359 + F::cast_from(3.0_f64) * t32822 * t8079 - t7357 * t8233 + t128970 - t128974 + t128975 - t128977 - t128979 - t128981 - t128983;
    t131092
}
