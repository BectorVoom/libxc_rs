//! MGGA_C_REVTPSS lxc pol — lxc_pol part 32 (v4rho3sigma_7) CSE chunk 1128/2056 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1128<F: Float>(t1513: F, t6998: F, t1544: F, t30: F, t1549: F, t7025: F, t1561: F, t7038: F, t1565: F, t7045: F, t1568: F, t1955: F) -> (F, F, F, F, F, F) {
    let t7738 = t6998 * t1513;
    let t7749 = t30 * t1544;
    let t7753 = t7025 * t1549;
    let t7755 = t7038 * t1561;
    let t7757 = t7045 * t1565;
    let t7766 = t1955 * t1568;
    (t7738, t7749, t7753, t7755, t7757, t7766)
}
