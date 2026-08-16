//! MGGA_C_REVTPSS lxc pol — lxc_pol part 51 (v4rho2sigma2_6) CSE chunk 1182/1200 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part51_v4rho2sigma2_6_chunk1182<F: Float>(t32099: F, t7898: F, t25082: F, t27153: F, t37110: F, t33913: F, t7316: F, t28173: F, t8568: F, t33974: F, t531: F, t2014: F, t7238: F) -> (F, F, F, F, F) {
    let t127302 = F::cast_from(3.0_f64) * t7898 * t32099;
    let t127305 = F::cast_from(6.0_f64) * t25082 * t37110 * t27153;
    let t127306 = t33913 * t7316;
    let t127308 = t8568 * t28173;
    let t127310 = t531 * t33974;
    let t127313 = F::cast_from(3.0_f64) * t2014 * t127310 * t7238;
    (t127302, t127305, t127306, t127308, t127313)
}
