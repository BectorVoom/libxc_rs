//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 2984/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2984<F: Float>(t49146: F, t543: F, t2782: F, t4100: F, t48475: F, t47423: F, t5741: F, t3923: F, t48105: F, t47371: F, t10026: F, t14141: F) -> (F, F, F, F, F, F) {
    let t49376 = t49146 * t543;
    let t49378 = t2782 * t4100 * t49376;
    let t49380 = t48475 * t543;
    let t49382 = t2782 * t4100 * t49380;
    let t49386 = t47423 * t5741;
    let t49393 = t48105 * t3923;
    let t49395 = t2782 * t47371 * t49393;
    let t49399 = t14141 * t10026;
    (t49376, t49378, t49382, t49386, t49395, t49399)
}
