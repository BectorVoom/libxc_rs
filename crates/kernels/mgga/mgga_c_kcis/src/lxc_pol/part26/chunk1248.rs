//! MGGA_C_KCIS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 1248/1397 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part26_v4rho3sigma_8_chunk1248<F: Float>(t54162: F, t8147: F, t2237: F, t556: F, t94424: F, t18210: F, t28402: F, t7898: F, t27345: F, t8151: F, t27348: F, t28544: F) -> (F, F, F, F, F, F, F, F) {
    let t98524 = t54162 * t8147;
    let t98525 = t2237 * t98524;
    let t98530 = t94424 * t556;
    let t98537 = t18210 * t28402;
    let t98538 = t7898 * t98537;
    let t98566 = t8151 * t27345;
    let t98568 = t8151 * t27348;
    let t98570 = t28544 * t27348;
    (t98524, t98525, t98530, t98537, t98538, t98566, t98568, t98570)
}
