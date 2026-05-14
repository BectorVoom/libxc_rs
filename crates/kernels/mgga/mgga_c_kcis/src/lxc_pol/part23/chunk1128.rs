//! MGGA_C_KCIS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 1128/1177 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part23_v4rho3sigma_5_chunk1128<F: Float>(t15866: F, t4160: F, t98034: F, t2002: F, t303: F, t94528: F, t1498: F, t5871: F, t1983: F, t3723: F, t3245: F, t8168: F, t12286: F, t6140: F, t2012: F, t4110: F) -> (F, F, F, F, F, F, F) {
    let t98706 = t4160 * t98034 * t15866;
    let t98709 = t303 * t94528 * t2002;
    let t98712 = t303 * t5871 * t1498;
    let t98715 = t303 * t1983 * t3723;
    let t98719 = t3245 * t8168;
    let t98721 = t12286 * t6140;
    let t98725 = t303 * t4110 * t2012;
    (t98706, t98709, t98712, t98715, t98719, t98721, t98725)
}
