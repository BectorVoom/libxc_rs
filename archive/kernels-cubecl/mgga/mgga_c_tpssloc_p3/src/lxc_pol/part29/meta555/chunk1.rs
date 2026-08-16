//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 29 (v4rho3sigma_5) CSE chunk 1957/2357 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1957<F: Float>(t1222: F, t8049: F, t5017: F, t7337: F, t1207: F, t1218: F, t2136: F, t24675: F, t24681: F, t24690: F, t24704: F, t27578: F, t27580: F, t27586: F, t27589: F, t488: F, t4974: F, t5014: F, t5030: F, t7339: F, t7345: F) -> (F, F, F) {
    let t27592 = t8049 * t1222;
    let t27598 = t7337 * t5017;
    let t27599 = t1207 * t27598;
    let t27602 = t24675 / F::cast_from(2304.0_f64) - t24681 + t27578 / F::cast_from(2304.0_f64) + F::cast_from(0.80745512188280781712e-3_f64) * t27580 * t2136 - t7345 * t4974 / F::cast_from(1152.0_f64) - t24690 / F::cast_from(864.0_f64) - t24704 + t27586 * t488 / F::cast_from(1536.0_f64) - t27589 * t488 / F::cast_from(288.0_f64) - t27592 / F::cast_from(432.0_f64) - t7345 * t5030 / F::cast_from(2304.0_f64) + t7339 * t5014 / F::cast_from(1536.0_f64) - t27599 * t1218 / F::cast_from(288.0_f64);
    (t27598, t27599, t27602)
}
