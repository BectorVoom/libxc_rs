//! GGA_C_FT97 lxc pol — lxc_pol part 18 (v4rho3sigma_3) CSE chunk 948/1396 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part18_v4rho3sigma_3_chunk948<F: Float>(t11119: F, t25752: F, t47: F, t9: F, t12486: F, t420: F, t422: F, t938: F, t379: F, t22572: F, t5569: F, t6441: F, t22798: F, t6426: F, t22797: F, t11247: F, t384: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t25753 = t11119 * t25752;
    let t25754 = t9 * t47;
    let t25755 = t420 * t12486;
    let t25756 = t25754 * t25755;
    let t25759 = t422 * t938;
    let t25760 = t25759 * t379;
    let t25768 = t5569 * t22572 * t6441;
    let t25770 = t6426 * t22798;
    let t25771 = t22797 * t25770;
    let t25774 = t11247 * t384;
    (t25753, t25754, t25755, t25756, t25759, t25760, t25768, t25770, t25771, t25774)
}
