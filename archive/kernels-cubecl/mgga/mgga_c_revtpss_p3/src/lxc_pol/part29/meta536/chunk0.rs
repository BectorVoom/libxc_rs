//! MGGA_C_REVTPSS lxc pol — lxc_pol part 29 (v4rho3sigma_4) CSE chunk 1868/2049 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1868<F: Float>(t92997: F, t92999: F, t93007: F, t93012: F, t93020: F, t26482: F, t93321: F, t25375: F, t95628: F, t136: F, t137: F, t2061: F) -> (F, F, F, F, F, F, F, F) {
    let t95674 = F::cast_from(0.15117061203111996147e0_f64) * t92997;
    let t95675 = F::cast_from(0.51384669507166276316e-2_f64) * t92999;
    let t95678 = F::cast_from(0.80328230880474379779e-6_f64) * t93007;
    let t95680 = F::cast_from(0.45178982497454656792e-6_f64) * t93012;
    let t95684 = F::cast_from(0.28900264064772933812e-2_f64) * t93020;
    let t95720 = t93321 * t26482;
    let t95722 = t25375 * t95628;
    let t95725 = t2061 * t136 * t137;
    (t95674, t95675, t95678, t95680, t95684, t95720, t95722, t95725)
}
