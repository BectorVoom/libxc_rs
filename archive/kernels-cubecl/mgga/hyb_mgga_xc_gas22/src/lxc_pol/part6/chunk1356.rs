//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 1356/1455 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk1356<F: Float>(t10629: F, t2311: F, t2314: F, t2315: F, t28937: F, t29440: F, t29442: F, t29445: F, t29448: F, t29451: F, t29454: F, t29458: F, t3430: F, t3436: F, t3445: F, t4211: F, t6759: F, t856: F, t8605: F, t8726: F) -> F {
    let t29526 = t29440 + t29442 + t29445 + t29448 - t29451 - t29454 - F::cast_from(0.6233709278045326953e3_f64) * t856 * t10629 * t2315 + t29458 - F::cast_from(0.34631718211362927518e2_f64) * t856 * t2311 * t28937 * t2314 - F::cast_from(0.5848223622634646207e0_f64) * t6759 * t4211 - F::cast_from(0.69263436422725855034e2_f64) * t3430 * t8605 - F::cast_from(0.69263436422725855034e2_f64) * t8726 * t3445 + F::cast_from(0.46785788981077169656e1_f64) * t8726 * t3436;
    t29526
}
