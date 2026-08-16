//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 1356/1455 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_rkernel_math::erf::{erf_approx};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk1356(t10629: f64, t2311: f64, t2314: f64, t2315: f64, t28937: f64, t29440: f64, t29442: f64, t29445: f64, t29448: f64, t29451: f64, t29454: f64, t29458: f64, t3430: f64, t3436: f64, t3445: f64, t4211: f64, t6759: f64, t856: f64, t8605: f64, t8726: f64) -> f64 {
    let t29526 = t29440 + t29442 + t29445 + t29448 - t29451 - t29454 - 0.6233709278045326953e3_f64 * t856 * t10629 * t2315 + t29458 - 0.34631718211362927518e2_f64 * t856 * t2311 * t28937 * t2314 - 0.5848223622634646207e0_f64 * t6759 * t4211 - 0.69263436422725855034e2_f64 * t3430 * t8605 - 0.69263436422725855034e2_f64 * t8726 * t3445 + 0.46785788981077169656e1_f64 * t8726 * t3436;
    t29526
}
