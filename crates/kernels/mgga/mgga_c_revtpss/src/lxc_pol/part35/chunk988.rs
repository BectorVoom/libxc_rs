//! MGGA_C_REVTPSS lxc pol — lxc_pol part 35 (v4rho3sigma_10) CSE chunk 988/1093 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part35_v4rho3sigma_10_chunk988<F: Float>(t30570: F, t508: F, t1518: F, t8065: F, t29494: F, t7488: F, t2107: F, t22483: F, t26161: F, t29498: F, t2051: F, t5883: F, t1312: F, t18245: F, t2055: F, t28653: F, t30138: F, t30143: F, t30553: F, t4248: F, t5920: F, t7359: F, t7889: F, t7983: F) -> (F, F, F, F, F, F, F) {
    let t30571 = t508 * t30570;
    let t30578 = t8065 * t1518;
    let t30581 = t7488 * t29494;
    let t30584 = t2107 * t22483;
    let t30586 = t26161 * t29498;
    let t30589 = t2051 * t5883;
    let t30612 = 2.0 * t1312 * t30570 + 4.0 * t1518 * t28653 + 2.0 * t18245 * t2055 + 4.0 * t2055 * t30138 + 2.0 * t2055 * t30143 + 4.0 * t4248 * t7983 + 2.0 * t5920 * t7359 + 4.0 * t7889 * t7983 + t30553 + 2.0 * t30589;
    (t30571, t30578, t30581, t30584, t30586, t30589, t30612)
}
