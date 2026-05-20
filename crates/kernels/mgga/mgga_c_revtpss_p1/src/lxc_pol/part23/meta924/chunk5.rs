//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2993/3317 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2993<F: Float>(t52037: F, t52955: F, t63338: F, t63340: F, t63342: F, t63361: F, t63371: F, t63447: F, t63453: F, t63459: F, t63464: F, t77559: F, t77561: F, t77566: F, t77570: F, t77575: F, t77581: F, t77586: F, t77590: F, t77594: F) -> F {
    let t79386 = -F::cast_from(0.33333333333333333333e-1_f64) * t63338 + F::cast_from(0.11111111111111111111e-1_f64) * t63340 + F::cast_from(0.92592592592592592592e-2_f64) * t63342 + F::cast_from(0.50000000000000000001e-1_f64) * t63361 - F::cast_from(0.33333333333333333334e-1_f64) * t63371 + t52955 - F::cast_from(0.74074074074074074073e-2_f64) * t52037 + F::cast_from(0.83333333333333333334e-2_f64) * t63447 - F::cast_from(0.74074074074074074073e-2_f64) * t63453 + F::cast_from(0.22222222222222222223e-1_f64) * t63459 + F::cast_from(0.55555555555555555553e-2_f64) * t77559 - F::cast_from(0.16666666666666666667e-1_f64) * t77561 + F::cast_from(0.11111111111111111111e0_f64) * t77566 - F::cast_from(0.27777777777777777778e-1_f64) * t77570 - F::cast_from(0.24691358024691358025e-1_f64) * t77575 - F::cast_from(0.11111111111111111111e-1_f64) * t63464 + F::cast_from(0.16666666666666666667e-1_f64) * t77581 - F::cast_from(0.55555555555555555555e-2_f64) * t77586 - F::cast_from(0.19999999999999999999e0_f64) * t77590 + F::cast_from(0.99999999999999999999e-1_f64) * t77594;
    t79386
}
