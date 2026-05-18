//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 378/1242 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk378<F: Float>(t102: F, t120: F, t1533: F, t118: F, t119: F, t331: F, t156: F, t497: F, t496: F, t1504: F, t506: F, t10: F) -> (F, F, F, F, F, F) {
    let t1536 = F::new(0.2923025e1) * t102 * t120 * t1533;
    let t1540 = t118 * t119 * t331 * t120 / F::new(9.0);
    let t1541 = t156 * t497;
    let t1542 = t496 * t1541;
    let t1544 = t506 * t1504;
    let t1545 = t10 * t1544;
    (t1536, t1540, t1541, t1542, t1544, t1545)
}
