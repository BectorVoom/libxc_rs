//! MGGA_C_REVTPSS lxc pol — lxc_pol part 25 (v4rho3sigma_0) CSE chunk 1159/1360 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part25_v4rho3sigma_0_chunk1159<F: Float>(t11239: F, t378: F, t1078: F, t1982: F, t1976: F, t3143: F, t3151: F, t3304: F, t3318: F, t7168: F, t1035: F, t7135: F) -> (F, F, F, F, F) {
    let t25669 = t378 * t11239;
    let t25671 = t1982 * t25669 * t1078;
    let t25672 = t3143 * t1976;
    let t25674 = t25672 * t3151 * t3304;
    let t25678 = t7168 * t3151 * t3318;
    let t25681 = t1035 * t7135;
    (t25671, t25672, t25674, t25678, t25681)
}
