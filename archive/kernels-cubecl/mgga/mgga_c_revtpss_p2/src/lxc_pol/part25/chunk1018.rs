//! MGGA_C_REVTPSS lxc pol — lxc_pol part 25 (v4rho3sigma_0) CSE chunk 1018/1360 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part25_v4rho3sigma_0_chunk1018<F: Float>(t1089: F, t11928: F, t1071: F, t1086: F, t994: F, t11869: F, t3316: F, t989: F, t1082: F, t11804: F, t11239: F, t11627: F) -> (F, F, F, F, F, F) {
    let t12150 = t11928 * t1089;
    let t12153 = t1086 * t1071;
    let t12154 = t994 * t12153;
    let t12157 = t11869 * t1089;
    let t12160 = t989 * t3316;
    let t12163 = t1082 * t11804;
    let t12166 = t11239 * t11627;
    (t12150, t12154, t12157, t12160, t12163, t12166)
}
