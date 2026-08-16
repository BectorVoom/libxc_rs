//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 29 (v4rho3sigma_5) CSE chunk 2324/2357 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2324<F: Float>(t15572: F, t24741: F, t15501: F, t24727: F, t3500: F, t7337: F, t15478: F, t15527: F, t15656: F, t15714: F, t24699: F, t24706: F, t24815: F, t27599: F, t27636: F, t27637: F, t3493: F, t3496: F, t3511: F, t3518: F, t7339: F, t7345: F, t8028: F, t8031: F, t86354: F) -> F {
    let t95617 = t24741 * t15572 / F::cast_from(1728.0_f64);
    let t95623 = t3500 * t24727 * t15501;
    let t95627 = t3500 * t7337 * t15501;
    let t95633 = F::cast_from(0.20186378047070195428e-3_f64) * t27636 * t27637 * t24815 * t3493 + F::cast_from(0.10093189023535097714e-3_f64) * t8031 * t24699 + F::cast_from(0.80745512188280781712e-3_f64) * t8028 * t24706 + F::cast_from(5.0_f64) / F::cast_from(6912.0_f64) * t24741 * t15714 - t24741 * t15478 / F::cast_from(1152.0_f64) - t95617 + t7339 * t15527 / F::cast_from(1536.0_f64) - t27599 * t3496 / F::cast_from(288.0_f64) - t95623 * t3511 / F::cast_from(144.0_f64) + t95627 * t3518 / F::cast_from(288.0_f64) - t86354 / F::cast_from(1728.0_f64) + F::cast_from(5.0_f64) / F::cast_from(1152.0_f64) * t7345 * t15656;
    t95633
}
