//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 32 (v4rho3sigma_8) CSE chunk 2324/2369 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2324(t1202: f64, t2132: f64, t2133: f64, t2136: f64, t24815: f64, t27636: f64, t27637: f64, t27655: f64, t27704: f64, t29600: f64, t29615: f64, t29644: f64, t29648: f64, t488: f64, t4950: f64, t5011: f64, t6144: f64, t7316: f64, t7321: f64, t8028: f64, t86149: f64, t95456: f64, t95459: f64, t95463: f64, t95465: f64, t95687: f64, t99767: f64) -> f64 {
    let t104220 = -0.20186378047070195428e-3_f64 * t86149 * t29644 + 0.10093189023535097714e-3_f64 * t86149 * t29648 - t95456 - t95459 - t95463 + t95465 + 0.10093189023535097714e-3_f64 * t7316 * t29615 + 0.16149102437656156342e-2_f64 * t8028 * t27655 - t95687 * t4950 / 1152.0_f64 - 0.20186378047070195428e-3_f64 * t27704 * t27655 - 0.10093189023535097714e-3_f64 * t2132 * t2133 * t6144 * t7321 - 0.10093189023535097714e-3_f64 * t2132 * t99767 * t2136 + 19.0_f64 / 864.0_f64 * t1202 * t29600 * t488 + 0.40372756094140390856e-3_f64 * t27636 * t27637 * t24815 * t5011;
    t104220
}
