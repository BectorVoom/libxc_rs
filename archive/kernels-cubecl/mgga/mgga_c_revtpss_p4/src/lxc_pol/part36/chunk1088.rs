//! MGGA_C_REVTPSS lxc pol — lxc_pol part 36 (v4rho3sigma_11) CSE chunk 1088/1378 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part36_v4rho3sigma_11_chunk1088<F: Float>(t1794: F, t6587: F, t1250: F, t3720: F, t1715: F, t20809: F, t1042: F, t5192: F, t6548: F, t12552: F, t24375: F, t12555: F) -> (F, F, F, F, F) {
    let t24751 = t6587 * t1794;
    let t24752 = t24751 * t1250;
    let t24753 = t3720 * t24752;
    let t24758 = t20809 * t1715;
    let t24759 = t1042 * t24758;
    let t24763 = F::cast_from(0.35089341735807877242e1_f64) * t5192 * t6548;
    let t24764 = t12552 * t24375;
    let t24765 = t24764 * t12555;
    (t24751, t24753, t24759, t24763, t24765)
}
