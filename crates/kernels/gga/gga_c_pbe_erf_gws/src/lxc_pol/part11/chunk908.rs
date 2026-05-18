//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 908/1302 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk908<F: Float>(t1275: F, t1293: F, t4659: F, t1285: F, t4661: F, t4358: F, t461: F, t18486: F, t18488: F, t18491: F, t18494: F, t18500: F, t18502: F, t18504: F, t18506: F, t7236: F, t7271: F) -> (F, F, F, F, F) {
    let t18515 = t1275 * t1275;
    let t18518 = F::new(0.57894567559743977359e3) * t4659 * t18515 * t1293;
    let t18527 = F::new(0.3103500882342370105e4) * t4659 * t1275 * t4661 * t1285;
    let t18528 = t4358 * t461;
    let t18529 = F::new(96.0) * t18528;
    let t18552 = -F::new(0.28769444444444444444e1) * t18486 + F::new(0.27618666666666666667e2) * t18488 - F::new(0.10229135802469135803e2) * t18491 + F::new(0.89504938271604938273e1) * t18494 + F::new(0.31310740740740740741e1) * t7271 + F::new(0.366775e-1) * t18500 - F::new(0.58684e0) * t18502 + F::new(0.65204444444444444445e0) * t18504 + F::new(0.5705388888888888889e0) * t18506 + F::new(0.13490888888888888889e1) * t7236;
    (t18515, t18518, t18527, t18529, t18552)
}
