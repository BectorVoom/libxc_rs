//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 3132/3221 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3132<F: Float>(t3446: F, t61064: F, t1176: F, t1714: F, t1184: F, t15293: F, t15382: F, t3439: F, t44424: F, t44439: F, t52074: F, t52076: F, t52081: F, t52084: F, t52086: F, t52089: F, t52092: F, t52109: F) -> F {
    let t64824 = t3446 * t61064;
    let t64825 = t1176 * t1714;
    let t64845 = F::cast_from(0.22222222222222222222e-2_f64) * t64824 * t64825 * t1184 * t15293 + F::cast_from(0.18518518518518518518e-3_f64) * t44424 + F::cast_from(0.18518518518518518518e-3_f64) * t44439 - F::cast_from(0.19753086419753086419e-2_f64) * t52074 + F::cast_from(0.14814814814814814814e-2_f64) * t52076 - F::cast_from(0.6172839506172839506e-3_f64) * t52081 + F::cast_from(0.74074074074074074072e-3_f64) * t52084 + F::cast_from(0.14814814814814814814e-2_f64) * t52086 - F::cast_from(0.14814814814814814814e-2_f64) * t64824 * t3439 * t1714 * t1184 * t15382 + F::cast_from(0.37037037037037037036e-3_f64) * t52089 - F::cast_from(0.74074074074074074072e-3_f64) * t52092 - F::cast_from(0.32921810699588477366e-3_f64) * t52109;
    t64845
}
