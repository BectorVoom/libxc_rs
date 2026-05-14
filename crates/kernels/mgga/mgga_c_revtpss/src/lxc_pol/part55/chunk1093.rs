//! MGGA_C_REVTPSS lxc pol — lxc_pol part 55 (v4rho2sigma2_10) CSE chunk 1093/1151 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part55_v4rho2sigma2_10_chunk1093<F: Float>(t28042: F, t94: F, t2056: F, t34261: F, t7367: F, t32392: F, t7984: F, t32394: F, t28704: F, t8634: F, t127381: F, t25082: F, t26405: F, t28173: F, t8698: F, t102019: F, t1936: F) -> (F, F, F, F, F, F, F, F) {
    let t128304 = t94 * t28042;
    let t128305 = t128304 * t2056;
    let t128306 = t34261 * t7367;
    let t128317 = 2.0 * t32392 * t7984;
    let t128319 = 2.0 * t32394 * t7984;
    let t128321 = 2.0 * t8634 * t28704;
    let t128324 = 3.0 * t25082 * t26405 * t127381;
    let t128326 = 3.0 * t8698 * t28173;
    let t128331 = t102019 * t1936;
    (t128305, t128306, t128317, t128319, t128321, t128324, t128326, t128331)
}
