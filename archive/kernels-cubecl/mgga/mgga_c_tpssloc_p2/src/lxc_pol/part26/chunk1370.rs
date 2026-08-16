//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 26 (v4rho3sigma_2) CSE chunk 1370/1384 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part26_v4rho3sigma_2_chunk1370<F: Float>(t11745: F, t24729: F, t2132: F, t24746: F, t86192: F, t11638: F, t11655: F, t11766: F, t11770: F, t2133: F, t2136: F, t24650: F, t24655: F, t24733: F, t3469: F, t475: F, t68: F, t7310: F, t7321: F, t7326: F, t7328: F, t7331: F, t7345: F, t83100: F, t86293: F, t86296: F) -> F {
    let t86299 = t24729 * t11745;
    let t86313 = t2132 * t86192 * t24746;
    let t86317 = -t24733 * t11770 / F::cast_from(512.0_f64) + F::cast_from(0.10093189023535097714e-3_f64) * t7326 * t7328 * t11638 * t68 * t475 - F::cast_from(0.60559134141210586284e-3_f64) * t86293 - F::cast_from(0.30279567070605293142e-3_f64) * t86296 * t7331 + t86299 / F::cast_from(384.0_f64) + F::cast_from(5.0_f64) / F::cast_from(1152.0_f64) * t7345 * t11655 - F::cast_from(0.30279567070605293142e-3_f64) * t2132 * t2133 * t3469 * t7321 - F::cast_from(0.30279567070605293142e-3_f64) * t24650 * t24655 - F::cast_from(0.10093189023535097714e-3_f64) * t2132 * t83100 * t2136 + F::cast_from(0.60559134141210586284e-3_f64) * t86313 - F::cast_from(7.0_f64) / F::cast_from(648.0_f64) * t7310 * t11766;
    t86317
}
