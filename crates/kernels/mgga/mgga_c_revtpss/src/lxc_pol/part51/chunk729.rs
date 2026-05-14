//! MGGA_C_REVTPSS lxc pol — lxc_pol part 51 (v4rho2sigma2_6) CSE chunk 729/1050 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part51_v4rho2sigma2_6_chunk729<F: Float>(t25610: F, t8521: F, t995: F, t342: F, t7135: F, t1071: F, t3140: F, t1078: F, t1982: F, t1976: F, t3057: F, t989: F, t11239: F, t378: F, t1035: F, t3046: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t25611 = t25610 * t8521;
    let t25629 = t995 * t8521;
    let t25634 = t342 * t7135;
    let t25638 = t1071 * t3140;
    let t25640 = t1982 * t25638 * t1078;
    let t25651 = t3057 * t1976;
    let t25658 = t989 * t1976;
    let t25669 = t378 * t11239;
    let t25681 = t1035 * t7135;
    let t25692 = t3046 * t1976;
    (t25611, t25629, t25634, t25638, t25640, t25651, t25658, t25669, t25681, t25692)
}
