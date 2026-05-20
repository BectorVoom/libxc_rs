//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3106/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3106<F: Float>(t1063: F, t16200: F, t3172: F, t15775: F, t3188: F, t16204: F, t16209: F, t11922: F, t11927: F, t15621: F, t11671: F, t4954: F) -> (F, F, F, F, F, F) {
    let t54438 = t1063 * t3172 * t16200;
    let t54440 = t3188 * t15775;
    let t54443 = t1063 * t3172 * t16204;
    let t54446 = t1063 * t3172 * t16209;
    let t54469 = t11927 * t11922 * t15621;
    let t54471 = t4954 * t11671;
    (t54438, t54440, t54443, t54446, t54469, t54471)
}
