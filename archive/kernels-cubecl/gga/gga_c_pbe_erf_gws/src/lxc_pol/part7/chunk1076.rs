//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1076/1242 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk1076<F: Float>(t102: F, t128: F, t16423: F, t505: F, t97: F, t120: F, t19083: F, t156: F, t496: F, t5744: F, t5772: F, t19344: F) -> (F, F, F, F, F) {
    let t19365 = F::cast_from(0.1753815e2_f64) * t102 * t128 * t16423;
    let t19367 = F::cast_from(1.0_f64) / t505 / t97;
    let t19373 = F::cast_from(0.2923025e1_f64) * t102 * t120 * t19083;
    let t19381 = t496 * t156 * t5744;
    let t19383 = t5772 * t120;
    let t19384 = t19383 * t19344;
    (t19365, t19367, t19373, t19381, t19384)
}
