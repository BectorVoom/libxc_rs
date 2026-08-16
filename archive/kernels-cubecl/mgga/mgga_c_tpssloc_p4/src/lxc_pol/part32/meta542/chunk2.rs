//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 32 (v4rho3sigma_8) CSE chunk 1889/2369 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1889<F: Float>(t1193: F, t8020: F, t1198: F, t2134: F, t24723: F, t24729: F, t24733: F, t24741: F, t27651: F, t27655: F, t27674: F, t4950: F, t4954: F, t4980: F, t4984: F, t5046: F, t7310: F, t7316: F, t7321: F, t8028: F, t8031: F, t8035: F) -> (F, F) {
    let t27677 = t8020 * t1193;
    let t27679 = -F::cast_from(0.10093189023535097714e-3_f64) * t27651 + F::cast_from(0.10093189023535097714e-3_f64) * t24723 - F::cast_from(0.10093189023535097714e-3_f64) * t2134 * t27655 + F::cast_from(0.10093189023535097714e-3_f64) * t7316 * t8035 - t24741 * t4950 / F::cast_from(2304.0_f64) - t24741 * t4954 / F::cast_from(2304.0_f64) + t24729 * t4980 / F::cast_from(768.0_f64) - t24733 * t4984 / F::cast_from(1536.0_f64) - t7310 * t5046 / F::cast_from(288.0_f64) + F::cast_from(0.80745512188280781712e-3_f64) * t8028 * t7321 + F::cast_from(0.10093189023535097714e-3_f64) * t8031 * t7321 + t27674 * t1198 / F::cast_from(108.0_f64) - t27677 / F::cast_from(108.0_f64);
    (t27677, t27679)
}
