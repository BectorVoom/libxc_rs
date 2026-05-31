//! MGGA_C_REVTPSS lxc pol — lxc_pol part 52 (v4rho2sigma2_7) CSE chunk 1274/1292 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part52_v4rho2sigma2_7_chunk1274<F: Float>(t102070: F, t1448: F, t28196: F, t34297: F, t25082: F, t27153: F, t37318: F, t32738: F, t98450: F, t4246: F, t8686: F, t1502: F, t32575: F) -> (F, F, F, F, F) {
    let t128917 = F::cast_from(6.0_f64) * t28196 * t102070 * t34297 * t1448;
    let t128920 = F::cast_from(3.0_f64) * t25082 * t37318 * t27153;
    let t128930 = F::cast_from(3.0_f64) * t98450 * t32738;
    let t128932 = t4246 * t8686;
    let t128933 = t1502 * t32575;
    (t128917, t128920, t128930, t128932, t128933)
}
