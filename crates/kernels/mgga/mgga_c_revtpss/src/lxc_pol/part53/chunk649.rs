//! MGGA_C_REVTPSS lxc pol — lxc_pol part 53 (v4rho2sigma2_8) CSE chunk 649/1089 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part53_v4rho2sigma2_8_chunk649<F: Float>(t30: F, t265: F, t393: F, t2163: F, t670: F, t7193: F, t2129: F, t45: F, t606: F, t7099: F, t1209: F, t2142: F, dens_threshold: F, rho0: F, zeta_threshold: F) -> (F, F, F, F) {
    let t31 = t30 <= zeta_threshold;
    let t120 = rho0 <= dens_threshold || t31;
    let t394 = t265 < t393;
    let t7591 = t2163 * t670;
    let t7594 = piecewise3(t394, 0.0, t7193);
    let t7599 = piecewise3(t120, t7099, t2129 * t606 / 2.0 + t7594 * t45 / 2.0);
    let t7602 = t1209 * t2142;
    (t7591, t7594, t7599, t7602)
}
