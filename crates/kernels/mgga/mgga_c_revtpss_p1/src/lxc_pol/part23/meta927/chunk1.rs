//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 3010/3317 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3010<F: Float>(t53252: F, t53253: F, t63338: F, t63340: F, t63342: F, t63361: F, t63371: F, t63447: F, t63453: F, t63459: F, t63464: F, t77559: F, t77561: F, t77566: F, t77570: F, t77575: F, t77581: F, t77586: F, t77590: F, t77594: F) -> F {
    let t80027 = -F::cast_from(0.59266666666666666668e-1_f64) * t63338 + F::cast_from(0.19755555555555555556e-1_f64) * t63340 + F::cast_from(0.16462962962962962963e-1_f64) * t63342 + F::cast_from(0.88900000000000000002e-1_f64) * t63361 - F::cast_from(0.59266666666666666668e-1_f64) * t63371 + t53252 - t53253 + F::cast_from(0.14816666666666666667e-1_f64) * t63447 - F::cast_from(0.13170370370370370371e-1_f64) * t63453 + F::cast_from(0.39511111111111111112e-1_f64) * t63459 + F::cast_from(0.9877777777777777778e-2_f64) * t77559 - F::cast_from(0.29633333333333333334e-1_f64) * t77561 + F::cast_from(0.19755555555555555556e0_f64) * t77566 - F::cast_from(0.49388888888888888889e-1_f64) * t77570 - F::cast_from(0.43901234567901234568e-1_f64) * t77575 - F::cast_from(0.19755555555555555556e-1_f64) * t63464 + F::cast_from(0.29633333333333333334e-1_f64) * t77581 - F::cast_from(0.9877777777777777778e-2_f64) * t77586 - F::cast_from(0.35560000000000000001e0_f64) * t77590 + F::new(0.1778e0) * t77594;
    t80027
}
