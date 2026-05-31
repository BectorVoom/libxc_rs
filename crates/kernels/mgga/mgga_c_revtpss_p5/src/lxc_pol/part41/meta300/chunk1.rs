//! MGGA_C_REVTPSS lxc pol — lxc_pol part 41 (v4rho3tau_4) CSE chunk 1067/1497 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk1067<F: Float>(t1036: F, t11997: F, t3141: F, t3144: F, t1035: F, t11239: F, t342: F, t3145: F, t334: F) -> (F, F, F, F, F) {
    let t11998 = t1036 * t11997;
    let t11999 = t3141 * t11998;
    let t12012 = t3144 * t11997;
    let t12013 = t3141 * t12012;
    let t12046 = t11239 * t1035;
    let t12047 = t342 * t12046;
    let t12050 = F::cast_from(1.0_f64) / t3145 / t334;
    (t11999, t12013, t12046, t12047, t12050)
}
