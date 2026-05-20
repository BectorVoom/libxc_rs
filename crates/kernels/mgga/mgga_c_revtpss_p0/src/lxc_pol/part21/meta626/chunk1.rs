//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2389/3259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2389<F: Float>(t2645: F, t775: F, t10779: F, t10786: F, t14931: F, t40583: F, t10773: F, t10811: F, t10696: F, t72: F, t245: F, t10729: F, t9775: F) -> (F, F, F, F, F, F) {
    let t40655 = t775 * t2645;
    let t40662 = t14931 * t10779 * t40583 * t10786;
    let t40669 = t10811 * t10773;
    let t40672 = t10696 * t72;
    let t40673 = t40672 * t245;
    let t40679 = t9775 * t10729;
    (t40655, t40662, t40669, t40672, t40673, t40679)
}
