//! GGA_C_GAPLOC lxc pol — lxc_pol part 39 (v4rhosigma3_4) CSE chunk 951/1028 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part39_v4rhosigma3_4_chunk951<F: Float>(t41139: F, t1445: F, t47225: F, t833: F, t47271: F, t41143: F, t43658: F, t43661: F, t43664: F, t43666: F, t43670: F, t43674: F, t43677: F, t43680: F, t47130: F, t701: F) -> (F, F) {
    let t47283 = 0.76685851907841499354e0 * t41139;
    let t47286 = t833 * t1445 * t47225;
    let t47290 = 0.11502877786176224903e2 * t833 * t1445 * t47271;
    let t47293 = -t47283 + 0.76685851907841499354e0 * t41143 + t43658 + t43661 + t43664 + 0.11502877786176224903e2 * t47286 + t47290 - 0.79445533226334281487e-1 * t43666 - t43670 - t43674 - 0.39722766613167140743e-1 * t43677 - t43680;
    let t47294 = t47130 * t701;
    (t47293, t47294)
}
