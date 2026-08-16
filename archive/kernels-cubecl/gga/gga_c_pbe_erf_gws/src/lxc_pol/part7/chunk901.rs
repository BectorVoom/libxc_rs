//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 901/1242 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk901<F: Float>(t1641: F, t1696: F, t16973: F, t11: F, t5089: F, t17003: F, t17007: F, t17011: F, t17016: F, t17020: F, t17024: F, t17026: F, t17028: F, t17030: F, t17032: F, t17034: F) -> (F, F, F, F) {
    let t17037 = F::cast_from(1.0_f64) / t1641 / t1696;
    let t17038 = t17037 * t16973;
    let t17040 = t11 * t5089 * t17038;
    let t17042 = -F::cast_from(0.45340000000000000002e-1_f64) * t17003 + F::cast_from(0.37783333333333333335e-2_f64) * t17007 + F::cast_from(0.5037777777777777778e-2_f64) * t17011 + F::cast_from(0.12594444444444444445e-1_f64) * t17016 - F::cast_from(0.4534e-1_f64) * t17020 + F::cast_from(0.6801e-1_f64) * t17024 - F::cast_from(0.10075555555555555556e-1_f64) * t17026 - F::cast_from(0.15113333333333333333e-1_f64) * t17028 + F::cast_from(0.15113333333333333333e-1_f64) * t17030 - F::cast_from(0.5037777777777777778e-2_f64) * t17032 + F::cast_from(0.10075555555555555556e-1_f64) * t17034 - F::cast_from(0.2518888888888888889e-1_f64) * t17040;
    (t17037, t17038, t17040, t17042)
}
