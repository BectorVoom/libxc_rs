//! MGGA_C_KCIS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 1154/1243 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part26_v4rho3sigma_8_chunk1154<F: Float>(t1394: F, t5667: F, t98409: F, t102051: F, t102054: F, t102057: F, t102061: F, t102064: F, t102068: F, t27595: F, t29514: F, t7978: F, t94901: F, t99152: F, t99154: F, t99157: F, t99173: F) -> (F, F) {
    let t102072 = t1394 * t98409 * t5667;
    let t102074 = -0.92835860883789062501e-5 * t94901 * t29514 + 0.51485339506172839507e-4 * t99152 + 0.20594135802469135803e-3 * t99154 - 0.77382407407407407408e-2 * t102051 - 0.41270617283950617283e-2 * t102054 + 0.15476481481481481481e-2 * t102057 - 0.185671721767578125e-4 * t27595 * t102061 + 0.20594135802469135803e-3 * t102064 - 0.34752604166666666667e-3 * t7978 * t102068 + 0.61905925925925925924e-2 * t102072 - t99157 + t99173;
    (t102072, t102074)
}
