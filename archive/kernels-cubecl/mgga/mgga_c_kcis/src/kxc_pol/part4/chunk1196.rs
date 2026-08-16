//! MGGA_C_KCIS kxc pol — kxc_pol part 4 (v3rho3_1) CSE chunk 1196/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_kxc_pol_part4_v3rho3_1_chunk1196<F: Float>(t187: F, t4731: F, t1684: F, t3005: F, t1831: F, t3551: F, t1835: F, t3006: F, t1219: F, t5234: F, t3569: F, t5237: F) -> (F, F, F, F, F, F, F) {
    let t15296 = t187 * t4731;
    let t15304 = t1684 * t3005;
    let t15307 = t1831 * t3551;
    let t15310 = t1835 * t3006;
    let t15317 = t5234 * t1219;
    let t15320 = t1831 * t3569;
    let t15323 = t5237 * t3551;
    (t15296, t15304, t15307, t15310, t15317, t15320, t15323)
}
