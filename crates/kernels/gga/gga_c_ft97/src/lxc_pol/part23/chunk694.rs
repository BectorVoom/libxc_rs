//! GGA_C_FT97 lxc pol — lxc_pol part 23 (v4rho3sigma_8) CSE chunk 694/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part23_v4rho3sigma_8_chunk694<F: Float>(t1168: F, t13830: F, t242: F, t5181: F, t713: F, t729: F, t17720: F, t17724: F, t17729: F, t17734: F, t17738: F, t17742: F, t17746: F, t17751: F, t17755: F, t17759: F, t17763: F) -> (F, F, F) {
    let t18233 = t13830 * t1168;
    let t18234 = t242 * t18233;
    let t18238 = t729 * t5181 * t713;
    let t18241 = 2.0 / 9.0 * t17720;
    let t18252 = -t18241 + t17724 / 3.0 + 2.0 / 3.0 * t17729 - 2.0 / 9.0 * t17734 - 4.0 / 3.0 * t17738 - 2.0 / 3.0 * t17742 - 2.0 * t17746 - 10.0 / 27.0 * t17751 + 8.0 / 9.0 * t17755 + 2.0 / 3.0 * t17759 + 2.0 / 9.0 * t17763;
    (t18234, t18238, t18252)
}
