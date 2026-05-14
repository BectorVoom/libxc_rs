//! MGGA_C_KCIS lxc pol — lxc_pol part 4 (v3rho3_1) CSE chunk 1055/1239 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part4_v3rho3_1_chunk1055<F: Float>(t1800: F, t3361: F, t1170: F, t3477: F, t5096: F, t3432: F, t5172: F, t3452: F, t10787: F, t5062: F, t14097: F, t5047: F, t5046: F, t10506: F, t251: F, t14611: F) -> (F, F, F, F, F, F, F) {
    let t15071 = t3361 * t1800;
    let t15072 = t1170 * t15071;
    let t15074 = t3477 * t5096;
    let t15076 = t5172 * t3432;
    let t15078 = t5172 * t3452;
    let t15080 = t10787 * t5062;
    let t15082 = t5047 * t14097;
    let t15083 = t5046 * t15082;
    let t15085 = t251 * t10506;
    let t15086 = t15085 * t14611;
    (t15072, t15074, t15076, t15078, t15080, t15083, t15086)
}
