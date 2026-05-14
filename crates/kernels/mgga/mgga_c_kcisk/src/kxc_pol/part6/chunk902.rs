//! MGGA_C_KCISK kxc pol — kxc_pol part 6 (v3rho3_3) CSE chunk 902/957 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_kxc_pol_part6_v3rho3_3_chunk902<F: Float>(t5770: F, t8378: F, t13110: F, t19100: F, t25590: F, t25601: F, t25609: F, t30569: F, t30572: F, t30592: F, t30595: F, t30599: F, t30603: F, t321: F, t25668: F, t6560: F) -> (F, F, F) {
    let t30716 = t5770 * t8378;
    let t30729 = -t13110 - 0.23744444444444444444e-1 * t19100 + 0.11872222222222222222e-1 * t25590 - 0.35616666666666666666e-1 * t25601 + 0.17808333333333333333e-1 * t25609 - 0.19787037037037037037e-1 * t30592 + 0.71233333333333333332e-1 * t30595 - 0.35616666666666666666e-1 * t30569 - 0.10685e0 * t30599 + 0.10685e0 * t30572 - 0.17808333333333333333e-1 * t30603;
    let t30731 = 0.62182e-1 * t30729 * t321;
    let t30734 = t25668 * t6560;
    (t30716, t30731, t30734)
}
