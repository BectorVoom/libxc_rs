//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 1087/1184 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk1087<F: Float>(t245: F, t149700: F, t149738: F, t149766: F, t149802: F, t149843: F, t149880: F, t149919: F, t149959: F, t149992: F, t150020: F, t151065: F, t151094: F, t151350: F, t151380: F, t151411: F, t152474: F, t1577: F, t18: F, t21: F, t33800: F, t35786: F, t363: F, t5: F, t7565: F, t920: F) -> F {
    let t246 = F::new(10000000.0) <= t245;
    let t152493 = piecewise3::<f64>(t246, F::new(0.0), t5 * (t149700 + t149738 + t149766 + t149802 + t149843 + t149880 + t149919 + t149959 + t149992 + t150020 + t151065 + t151094 + t151350 + t151380 + t151411 + t152474) * t21 / F::new(4.0) + t5 * t35786 * t363 / F::new(4.0) + t5 * t33800 * t920 / F::new(4.0) + t5 * t7565 * t18 * t1577 / F::new(2.0));
    t152493
}
