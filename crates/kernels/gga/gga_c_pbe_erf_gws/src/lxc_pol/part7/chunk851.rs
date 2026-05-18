//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 851/1242 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk851<F: Float>(t1371: F, t553: F, t6016: F, t1331: F, t8: F, t147: F, t551: F, t6038: F, t6041: F, t1354: F, t837: F, t6006: F) -> (F, F, F, F, F, F, F, F) {
    let t16460 = t6016 * t1371 * t553;
    let t16463 = F::new(1.0) / t8 / t1331;
    let t16465 = t16463 * t147 * t551;
    let t16467 = F::new(0.74395492895254307406e-5) * t16465 * t553;
    let t16468 = t6038 * t553;
    let t16471 = F::new(0.1035981803916141664e0) * t6041 * t553;
    let t16474 = t837 * t1354 * t551 * t553;
    let t16477 = t6006 * t1371 * t553;
    (t16460, t16463, t16465, t16467, t16468, t16471, t16474, t16477)
}
