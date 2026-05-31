//! GGA_C_GAPLOC lxc pol — lxc_pol part 49 (v4rhosigma3_14) CSE chunk 610/1217 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part49_v4rhosigma3_14_chunk610<F: Float>(t10205: F, t471: F, t3334: F, t64: F, t2748: F, t871: F, t9097: F, t9100: F, t9113: F, t9115: F) -> (F, F, F, F, F, F, F) {
    let t10206 = t10205 * t471;
    let t10208 = F::cast_from(4.0_f64) / F::cast_from(3.0_f64) * t3334 * t64;
    let t10209 = t2748 * t871;
    let t10211 = F::cast_from(7.0_f64) / F::cast_from(256.0_f64) * t9097;
    let t10212 = F::cast_from(21.0_f64) / F::cast_from(8192.0_f64) * t9100;
    let t10213 = F::cast_from(7.0_f64) / F::cast_from(8192.0_f64) * t9113;
    let t10214 = F::cast_from(7.0_f64) / F::cast_from(768.0_f64) * t9115;
    let t10215 = t10206 - t10208 + t10209 / F::cast_from(2.0_f64) - t10211 + t10212 - t10213 + t10214;
    (t10206, t10208, t10211, t10212, t10213, t10214, t10215)
}
