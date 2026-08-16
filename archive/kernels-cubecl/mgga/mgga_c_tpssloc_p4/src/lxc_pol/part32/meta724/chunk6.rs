//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 32 (v4rho3sigma_8) CSE chunk 2324/2369 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2324<F: Float>(t1202: F, t2132: F, t2133: F, t2136: F, t24815: F, t27636: F, t27637: F, t27655: F, t27704: F, t29600: F, t29615: F, t29644: F, t29648: F, t488: F, t4950: F, t5011: F, t6144: F, t7316: F, t7321: F, t8028: F, t86149: F, t95456: F, t95459: F, t95463: F, t95465: F, t95687: F, t99767: F) -> F {
    let t104220 = -F::cast_from(0.20186378047070195428e-3_f64) * t86149 * t29644 + F::cast_from(0.10093189023535097714e-3_f64) * t86149 * t29648 - t95456 - t95459 - t95463 + t95465 + F::cast_from(0.10093189023535097714e-3_f64) * t7316 * t29615 + F::cast_from(0.16149102437656156342e-2_f64) * t8028 * t27655 - t95687 * t4950 / F::cast_from(1152.0_f64) - F::cast_from(0.20186378047070195428e-3_f64) * t27704 * t27655 - F::cast_from(0.10093189023535097714e-3_f64) * t2132 * t2133 * t6144 * t7321 - F::cast_from(0.10093189023535097714e-3_f64) * t2132 * t99767 * t2136 + F::cast_from(19.0_f64) / F::cast_from(864.0_f64) * t1202 * t29600 * t488 + F::cast_from(0.40372756094140390856e-3_f64) * t27636 * t27637 * t24815 * t5011;
    t104220
}
