//! MGGA_C_REVTPSS lxc pol — lxc_pol part 55 (v4rho2sigma2_10) CSE chunk 979/1151 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part55_v4rho2sigma2_10_chunk979<F: Float>(t34132: F, t34166: F, t118: F, t7935: F, t8698: F, t4248: F, t8641: F, t7732: F, t1936: F, t8065: F, t651: F, t7898: F, t8715: F, t1955: F, t8085: F, t28911: F, t7925: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t34167 = t34132 + t34166;
    let t34168 = t118 * t34167;
    let t34191 = t8698 * t7935;
    let t34193 = 2.0 * t4248 * t8641;
    let t34195 = 2.0 * t7732 * t8641;
    let t34196 = t8065 * t1936;
    let t34198 = 2.0 * t651 * t34196;
    let t34203 = t7898 * t8715;
    let t34204 = t1955 * t8085;
    let t34212 = t28911 * t7925;
    (t34167, t34168, t34191, t34193, t34195, t34196, t34198, t34203, t34204, t34212)
}
