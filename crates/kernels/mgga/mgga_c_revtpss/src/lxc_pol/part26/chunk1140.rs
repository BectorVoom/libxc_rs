//! MGGA_C_REVTPSS lxc pol — lxc_pol part 26 (v4rho3sigma_1) CSE chunk 1140/1225 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part26_v4rho3sigma_1_chunk1140<F: Float>(t7264: F, t9901: F, t7271: F, t9986: F, t9893: F, t25997: F, t9905: F, t533: F, t816: F, t92993: F, t7259: F, t9709: F) -> (F, F, F, F, F, F) {
    let t94462 = t7264 * t9901;
    let t94464 = t7271 * t9986;
    let t94466 = t7264 * t9893;
    let t94468 = t25997 * t9905;
    let t94471 = t92993 * t533 * t816;
    let t94473 = t7259 * t9709;
    (t94462, t94464, t94466, t94468, t94471, t94473)
}
