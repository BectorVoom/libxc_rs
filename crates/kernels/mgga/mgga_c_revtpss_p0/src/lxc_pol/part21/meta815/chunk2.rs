//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2987/3259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2987<F: Float>(t11922: F, t11927: F, t15621: F, t11671: F, t4954: F, t16068: F, t999: F, t11249: F, t4866: F) -> (F, F, F, F) {
    let t54469 = t11927 * t11922 * t15621;
    let t54471 = t4954 * t11671;
    let t54474 = t16068 * t999;
    let t54479 = t4866 * t11249;
    (t54469, t54471, t54474, t54479)
}
