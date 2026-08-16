//! MGGA_C_REVTPSS lxc pol — lxc_pol part 55 (v4rho2sigma2_10) CSE chunk 1237/1306 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part55_v4rho2sigma2_10_chunk1237<F: Float>(t128304: F, t2056: F, t34261: F, t7367: F, t32392: F, t7984: F, t32394: F, t28704: F, t8634: F, t127381: F, t25082: F, t26405: F) -> (F, F, F, F, F, F) {
    let t128305 = t128304 * t2056;
    let t128306 = t34261 * t7367;
    let t128317 = F::cast_from(2.0_f64) * t32392 * t7984;
    let t128319 = F::cast_from(2.0_f64) * t32394 * t7984;
    let t128321 = F::cast_from(2.0_f64) * t8634 * t28704;
    let t128324 = F::cast_from(3.0_f64) * t25082 * t26405 * t127381;
    (t128305, t128306, t128317, t128319, t128321, t128324)
}
