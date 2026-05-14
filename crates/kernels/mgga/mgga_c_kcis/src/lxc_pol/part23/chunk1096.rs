//! MGGA_C_KCIS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 1096/1177 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part23_v4rho3sigma_5_chunk1096<F: Float>(t27553: F, t5897: F, t4188: F, t8182: F, t4190: F, t4189: F, t6048: F, t7962: F, t28450: F, t4142: F, t1307: F, t16681: F, t5709: F, t3805: F, t5885: F, t3797: F, t5701: F) -> (F, F, F, F, F, F, F) {
    let t97990 = t5897 * t27553;
    let t97991 = t8182 * t4188;
    let t97993 = 2.0 * t97991 * t4190;
    let t97996 = 4.0 * t4189 * t7962 * t6048;
    let t97997 = t4142 * t28450;
    let t98002 = t5709 * t16681 * t1307;
    let t98006 = t5709 * t5885 * t3805;
    let t98010 = t5701 * t5885 * t3797;
    (t97990, t97993, t97996, t97997, t98002, t98006, t98010)
}
