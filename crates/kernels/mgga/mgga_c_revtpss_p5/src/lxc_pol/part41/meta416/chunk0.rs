//! MGGA_C_REVTPSS lxc pol — lxc_pol part 41 (v4rho3tau_4) CSE chunk 1468/1497 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk1468<F: Float>(t31027: F, t8355: F, t28036: F, t8259: F, t1513: F, t31039: F, t658: F, t8268: F, t4287: F, t31032: F, t8358: F, t1504: F, t665: F) -> (F, F, F, F, F, F, F) {
    let t31259 = t31027 * t8355;
    let t31261 = t8259 * t28036;
    let t31264 = t31039 * t1513;
    let t31267 = t1513 * t658;
    let t31268 = t8268 * t31267;
    let t31271 = t8259 * t4287;
    let t31274 = t31032 * t8358;
    let t31276 = t1504 * t665;
    (t31259, t31261, t31264, t31268, t31271, t31274, t31276)
}
