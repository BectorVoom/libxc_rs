//! MGGA_C_KCIS kxc pol — kxc_pol part 4 (v3rho3_1) CSE chunk 1049/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_kxc_pol_part4_v3rho3_1_chunk1049<F: Float>(t13250: F, t3210: F, t13172: F, t4793: F, t9425: F, t5042: F, t922: F, t3202: F, t3200: F, t1767: F, t3219: F, t3218: F) -> (F, F, F, F, F) {
    let t13251 = t3210 * t13250;
    let t13252 = t13172 * t13251;
    let t13254 = t9425 * t4793;
    let t13256 = t5042 * t922;
    let t13257 = t3202 * t13256;
    let t13258 = t3200 * t13257;
    let t13260 = t1767 * t3219;
    let t13261 = t3218 * t13260;
    (t13252, t13254, t13258, t13260, t13261)
}
