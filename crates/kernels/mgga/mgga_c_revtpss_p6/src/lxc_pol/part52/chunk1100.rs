//! MGGA_C_REVTPSS lxc pol — lxc_pol part 52 (v4rho2sigma2_7) CSE chunk 1100/1292 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part52_v4rho2sigma2_7_chunk1100<F: Float>(t2007: F, t7983: F, t7984: F, t8634: F, t2055: F, t7883: F, t7359: F, t7742: F, t1907: F, t2033: F, t28286: F, t28196: F) -> (F, F, F, F, F, F, F) {
    let t34279 = t2007 * t7983;
    let t34285 = F::cast_from(2.0_f64) * t8634 * t7984;
    let t34290 = t7883 * t2055;
    let t34294 = F::cast_from(2.0_f64) * t7359 * t7742;
    let t34297 = t2033 * t1907;
    let t34298 = t28286 * t34297;
    let t34300 = F::cast_from(2.0_f64) * t28196 * t34298;
    (t34279, t34285, t34290, t34294, t34297, t34298, t34300)
}
