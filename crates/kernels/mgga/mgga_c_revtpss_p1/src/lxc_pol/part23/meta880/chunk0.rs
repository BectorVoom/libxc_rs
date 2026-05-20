//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2788/3317 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2788<F: Float>(t1882: F, t2482: F, t4104: F, t5767: F, t1892: F, t5658: F, t14230: F, t2782: F, t48083: F, t4086: F, t543: F, t10073: F, t22365: F) -> (F, F, F, F) {
    let t74908 = t2482 * t5767 * t1882 * t4104;
    let t74922 = t1892 * t5658;
    let t74935 = t2782 * t48083 * t14230;
    let t74943 = t2782 * t4086 * t74922 * t543;
    let t74945 = t10073 * t22365;
    (t74908, t74935, t74943, t74945)
}
