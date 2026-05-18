//! MGGA_C_REVTPSS lxc pol — lxc_pol part 55 (v4rho2sigma2_10) CSE chunk 1266/1306 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part55_v4rho2sigma2_10_chunk1266<F: Float>(t4246: F, t8686: F, t1502: F, t32575: F, t34241: F, t531: F, t2014: F, t7238: F, t1448: F, t25082: F, t28286: F, t34301: F) -> (F, F, F, F) {
    let t128932 = t4246 * t8686;
    let t128933 = t1502 * t32575;
    let t128934 = t531 * t34241;
    let t128937 = F::new(3.0) * t2014 * t128934 * t7238;
    let t128945 = F::new(6.0) * t25082 * t28286 * t34301 * t1448;
    (t128932, t128933, t128937, t128945)
}
