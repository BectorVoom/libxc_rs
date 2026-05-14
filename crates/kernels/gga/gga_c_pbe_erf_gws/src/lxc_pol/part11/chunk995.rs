//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 995/1141 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk995<F: Float>(t12355: F, t2678: F, t10534: F, t3354: F, t3465: F, t2672: F, t11: F, t1691: F, t47969: F, t625: F, t5089: F, t1714: F, t25: F, t40962: F, t40989: F, t5061: F, t657: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t47975 = t2678 * t12355;
    let t47979 = t10534 * t3354;
    let t47983 = t3465 * t3354;
    let t47987 = t2672 * t12355;
    let t47994 = t11 * t1691 * t47969;
    let t47997 = t11 * t1691 * t47975;
    let t48000 = t11 * t625 * t47983;
    let t48003 = t11 * t625 * t47987;
    let t48006 = t11 * t5089 * t47979;
    let t48008 = -0.88888888888888888888e-2 * t25 * t1714 * t47975 - 0.17777777777777777778e-1 * t25 * t5061 * t47979 - 0.24e0 * t25 * t657 * t47983 + 0.53333333333333333332e-1 * t25 * t657 * t47987 + 0.95977777777777777777e-1 * t40962 - 0.28793333333333333333e0 * t40989 + 0.86380000000000000002e0 * t47994 - 0.9597777777777777778e-1 * t47997 - 0.12957e1 * t48000 + 0.28793333333333333333e0 * t48003 - 0.23994444444444444446e0 * t48006;
    (t47975, t47979, t47983, t47987, t47994, t47997, t48000, t48003, t48006, t48008)
}
