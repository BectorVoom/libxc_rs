//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 38 (v4rho3tau_2) CSE chunk 1284/1335 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part38_v4rho3tau_2_chunk1284<F: Float>(t12189: F, t1811: F, t1358: F, t5231: F, t16123: F, t554: F, t1815: F, t3862: F, t3726: F, t5227: F, t119: F, t16018: F) -> (F, F, F, F, F, F) {
    let t16341 = t12189 * t1811;
    let t16346 = F::cast_from(7.0_f64) / F::cast_from(2304.0_f64) * t5231 * t1358;
    let t16347 = t16123 * t554;
    let t16350 = t1815 * t3862;
    let t16354 = F::cast_from(7.0_f64) / F::cast_from(72.0_f64) * t3726 * t5227;
    let t16355 = t119 * t16018;
    (t16341, t16346, t16347, t16350, t16354, t16355)
}
