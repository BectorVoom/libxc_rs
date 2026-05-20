//! MGGA_C_REVTPSS lxc pol — lxc_pol part 52 (v4rho2sigma2_7) CSE chunk 1092/1292 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part52_v4rho2sigma2_7_chunk1092<F: Float>(t4248: F, t8641: F, t7732: F, t1936: F, t8065: F, t651: F, t7898: F, t8715: F, t1955: F, t8085: F, t28911: F, t7925: F) -> (F, F, F, F, F, F, F) {
    let t34193 = F::new(2.0) * t4248 * t8641;
    let t34195 = F::new(2.0) * t7732 * t8641;
    let t34196 = t8065 * t1936;
    let t34198 = F::new(2.0) * t651 * t34196;
    let t34203 = t7898 * t8715;
    let t34204 = t1955 * t8085;
    let t34212 = t28911 * t7925;
    (t34193, t34195, t34196, t34198, t34203, t34204, t34212)
}
