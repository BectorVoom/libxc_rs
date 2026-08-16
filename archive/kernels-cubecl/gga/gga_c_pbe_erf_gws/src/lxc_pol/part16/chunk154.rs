//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 16 (v4rho3sigma_4) CSE chunk 154/1361 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part16_v4rho3sigma_4_chunk154<F: Float>(t27: F, t13: F, t14: F, t1: F, t119: F, t155: F, t156: F, t4: F) -> (F, F, F, F, F, F, F, F) {
    let t387 = t27 * t27;
    let t388 = F::cast_from(1.0_f64) / t387;
    let t389 = t13 * t388;
    let t390 = F::cast_from(1.0_f64) / t14;
    let t391 = t390 * t1;
    let t392 = t119 * t155;
    let t393 = t391 * t392;
    let t395 = t4 * t156;
    (t387, t388, t389, t390, t391, t392, t393, t395)
}
