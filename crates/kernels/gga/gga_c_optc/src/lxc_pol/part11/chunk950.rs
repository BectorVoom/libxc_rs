//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 950/1451 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk950<F: Float>(t17449: F, t8697: F, t8700: F, t1102: F, t4224: F, t5219: F, t4305: F, t5264: F, t11604: F, t11761: F, t1220: F, t14864: F, t17363: F, t17425: F, t17429: F, t17431: F, t17433: F, t17435: F, t17438: F, t17440: F, t17443: F, t17447: F, t4230: F, t5233: F, t9229: F) -> (F, F, F, F, F, F) {
    let t17450 = t8697 * t17449;
    let t17451 = t17450 * t8700;
    let t17453 = F::cast_from(0.1025389702100779493e4_f64) * t1102 * t17451;
    let t17454 = t4224 * t5219;
    let t17456 = F::cast_from(0.35089340384731224426e1_f64) * t1102 * t17454;
    let t17460 = F::cast_from(0.35089340384731224426e1_f64) * t4305 * t5264;
    let t17463 = t17363 + t17425 + t17429 + t17431 + t17433 + t17435 - t17438 + t1220 * t17440 + F::new(14.0) / F::new(27.0) * t1220 * t17443 + t9229 - t17447 - t14864 / F::new(3.0) - t17453 + t17456 - F::new(100.0) / F::new(81.0) * t11604 + F::new(8.0) / F::new(9.0) * t11761 + t17460 + F::new(8.0) / F::new(3.0) * t4230 * t5233;
    (t17451, t17453, t17454, t17456, t17460, t17463)
}
