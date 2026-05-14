//! MGGA_C_KCIS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 1120/1177 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part23_v4rho3sigma_5_chunk1120<F: Float>(t1014: F, t28528: F, t54162: F, t8147: F, t2237: F, t15815: F, t303: F, t7931: F, t556: F, t94424: F, t15883: F, t5661: F, t18210: F, t28402: F, t7898: F, t1983: F, t4137: F) -> (F, F, F, F, F, F, F, F, F) {
    let t98522 = t1014 * t28528;
    let t98524 = t54162 * t8147;
    let t98525 = t2237 * t98524;
    let t98528 = t303 * t7931 * t15815;
    let t98530 = t94424 * t556;
    let t98532 = t5661 * t98530 * t15883;
    let t98537 = t18210 * t28402;
    let t98538 = t7898 * t98537;
    let t98543 = t303 * t1983 * t4137;
    (t98522, t98524, t98525, t98528, t98530, t98532, t98537, t98538, t98543)
}
