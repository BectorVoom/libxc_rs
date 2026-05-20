//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2911/3317 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2911<F: Float>(t52037: F, t52346: F, t63338: F, t63340: F, t63342: F, t63361: F, t63371: F, t63447: F, t63453: F, t63459: F, t63464: F, t77559: F, t77561: F, t77566: F, t77570: F, t77575: F, t77581: F, t77586: F, t77590: F, t77594: F) -> F {
    let t77596 = -F::cast_from(0.37083333333333333333e-1_f64) * t63338 + F::cast_from(0.12361111111111111111e-1_f64) * t63340 + F::cast_from(0.10300925925925925926e-1_f64) * t63342 + F::cast_from(0.55625000000000000001e-1_f64) * t63361 - F::cast_from(0.37083333333333333334e-1_f64) * t63371 + t52346 - F::cast_from(0.82407407407407407407e-2_f64) * t52037 + F::cast_from(0.92708333333333333334e-2_f64) * t63447 - F::cast_from(0.82407407407407407408e-2_f64) * t63453 + F::cast_from(0.24722222222222222223e-1_f64) * t63459 + F::cast_from(0.61805555555555555553e-2_f64) * t77559 - F::cast_from(0.18541666666666666667e-1_f64) * t77561 + F::cast_from(0.12361111111111111111e0_f64) * t77566 - F::cast_from(0.30902777777777777778e-1_f64) * t77570 - F::cast_from(0.27469135802469135803e-1_f64) * t77575 - F::cast_from(0.12361111111111111111e-1_f64) * t63464 + F::cast_from(0.18541666666666666667e-1_f64) * t77581 - F::cast_from(0.61805555555555555555e-2_f64) * t77586 - F::cast_from(0.22249999999999999999e0_f64) * t77590 + F::new(0.11125e0) * t77594;
    t77596
}
