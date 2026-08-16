//! MGGA_C_REVTPSS lxc pol — lxc_pol part 20 (v4rho4_0) CSE chunk 1012/1798 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1012<F: Float>(t10652: F, t2723: F, t4503: F, t2782: F, t2760: F, t822: F, t2718: F, t860: F, t2722: F, t836: F) -> (F, F, F, F, F) {
    let t10654 = t4503 * t10652 * t2723;
    let t10655 = t2782 * t10654;
    let t10657 = t822 * t2760;
    let t10661 = t2718 * t860;
    let t10665 = t2722 * t836;
    (t10654, t10655, t10657, t10661, t10665)
}
