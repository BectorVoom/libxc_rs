//! MGGA_C_REVTPSS lxc pol — lxc_pol part 55 (v4rho2sigma2_10) CSE chunk 833/1306 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part55_v4rho2sigma2_10_chunk833<F: Float>(t1031: F, t3800: F, t498: F, t1207: F, t1275: F, t4171: F, t602: F, t1466: F, t2246: F) -> (F, F, F, F, F, F) {
    let t11238 = t1031 * t1031;
    let t11239 = F::cast_from(1.0_f64) / t11238;
    let t12587 = F::cast_from(1.0_f64) / t3800 / t498;
    let t12625 = t1207 * t1207;
    let t12626 = F::cast_from(1.0_f64) / t12625;
    let t13180 = t1275 * t1275;
    let t13181 = F::cast_from(1.0_f64) / t13180;
    let t13269 = t4171 * t602;
    let t13272 = t1466 * t2246;
    (t11239, t12587, t12626, t13181, t13269, t13272)
}
