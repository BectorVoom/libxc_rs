//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 826/1141 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk826<F: Float>(t4358: F, t461: F, t18486: F, t18488: F, t18491: F, t18494: F, t18500: F, t18502: F, t18504: F, t18506: F, t7236: F, t7271: F, t449: F, t456: F, t470: F, t1272: F, t1289: F, t13: F, t18515: F, t4661: F) -> (F, F, F, F) {
    let t18528 = t4358 * t461;
    let t18529 = 96.0 * t18528;
    let t18552 = -0.28769444444444444444e1 * t18486 + 0.27618666666666666667e2 * t18488 - 0.10229135802469135803e2 * t18491 + 0.89504938271604938273e1 * t18494 + 0.31310740740740740741e1 * t7271 + 0.366775e-1 * t18500 - 0.58684e0 * t18502 + 0.65204444444444444445e0 * t18504 + 0.5705388888888888889e0 * t18506 + 0.13490888888888888889e1 * t7236;
    let t18556 = 0.58482233974552040708e0 * t470 * t449 * t18552 * t456;
    let t18562 = 0.620700176468474021e4 * t13 / t1289 / t1272 * t18515 * t4661;
    (t18529, t18552, t18556, t18562)
}
