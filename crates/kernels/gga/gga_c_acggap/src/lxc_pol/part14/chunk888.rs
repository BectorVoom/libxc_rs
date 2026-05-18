//! GGA_C_ACGGAP lxc pol — lxc_pol part 14 (v4rho3sigma_6) CSE chunk 888/1223 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part14_v4rho3sigma_6_chunk888<F: Float>(t7538: F, t7720: F, t7724: F, t30228: F, t601: F, t30174: F, t151: F, t56: F, t593: F, t606: F, t30225: F, t425: F) -> (F, F, F, F, F, F) {
    let t30655 = t7538 * t7720;
    let t30657 = t7538 * t7724;
    let t30658 = F::new(0.32155513588552302729e-3) * t30657;
    let t30663 = t30228 * t601;
    let t30664 = F::new(0.19293308153131381638e-2) * t30663;
    let t30665 = F::new(1.0) / t30174;
    let t30668 = t151 * t593 * t30665 * t56;
    let t30669 = t30668 * t601;
    let t30670 = F::new(0.36014175219178579057e-1) * t30669;
    let t30671 = t30668 * t606;
    let t30672 = F::new(0.52832795046534975474e-1) * t30671;
    let t30673 = t30225 * t425;
    (t30655, t30658, t30664, t30670, t30672, t30673)
}
