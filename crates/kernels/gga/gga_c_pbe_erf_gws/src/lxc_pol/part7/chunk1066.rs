//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1066/1242 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk1066<F: Float>(t19241: F, t496: F, t1: F, t119: F, t6045: F, t18483: F, t5773: F, t1504: F, t299: F, t799: F, t1552: F, t1563: F, t19: F) -> (F, F, F, F, F) {
    let t19242 = t496 * t19241;
    let t19247 = t6045 * t1 * t119;
    let t19249 = F::cast_from(0.16239027777777777777e1_f64) * param_hyb_omega_0 * t18483 * t5773 * t19247;
    let t19253 = t799 * t299 * t1504;
    let t19254 = t1552 * t1563 * t19 * t19253;
    (t19242, t19247, t19249, t19253, t19254)
}
