//! MGGA_C_REVTPSS lxc pol — lxc_pol part 52 (v4rho2sigma2_7) CSE chunk 1227/1292 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part52_v4rho2sigma2_7_chunk1227<F: Float>(t118: F, t128069: F, t128193: F, t28166: F, t8697: F, t28168: F, t13648: F, t2014: F, t8714: F, t127545: F, t127547: F, t127549: F, t127550: F, t127556: F, t127559: F, t2089: F, t28030: F, t28160: F, t32322: F, t32389: F, t4297: F, t7378: F, t7474: F, t7725: F, t8111: F) -> F {
    let t128195 = t118 * (t128069 + t128193);
    let t128196 = t8697 * t28166;
    let t128198 = F::cast_from(6.0_f64) * t128196 * t28168;
    let t128200 = t2014 * t8714 * t13648;
    let t128201 = -t2089 * t28160 - F::cast_from(2.0_f64) * t28030 * t7378 - t32322 * t8111 - F::cast_from(2.0_f64) * t32389 * t4297 - t7474 * t7725 - t127545 - t127547 - t127549 - t127550 - t127556 + t127559 - t128195 + t128198 - t128200;
    t128201
}
