//! MGGA_C_REVTPSS lxc pol — lxc_pol part 36 (v4rho3sigma_11) CSE chunk 1270/1378 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part36_v4rho3sigma_11_chunk1270<F: Float>(t25266: F, t6019: F, t6024: F, t93054: F, t18622: F, t25245: F, t5989: F, t92978: F, t25277: F, t5985: F, t18352: F, t1945: F, t807: F) -> (F, F, F, F, F, F) {
    let t106063 = t25266 * t6019;
    let t106065 = t93054 * t6024;
    let t106080 = t25245 * t18622;
    let t106082 = t92978 * t5989;
    let t106090 = t25277 * t5985;
    let t106102 = t807 * t1945 * t18352;
    (t106063, t106065, t106080, t106082, t106090, t106102)
}
