//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1087/1302 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk1087<F: Float>(t11: F, t47401: F, t571: F, t12345: F, t2554: F, t10777: F, t3346: F, t2560: F, t10783: F, t17728: F, t1856: F, t25: F, t40105: F, t40107: F, t40163: F, t40213: F, t47348: F, t47414: F, t5264: F, t606: F) -> (F, F, F, F, F, F) {
    let t47430 = t11 * t571 * t47401;
    let t47438 = t2554 * t12345;
    let t47442 = t10777 * t3346;
    let t47446 = t2560 * t12345;
    let t47450 = t10783 * t3346;
    let t47458 = -F::cast_from(0.35991666666666666667e-1_f64) * t47430 + F::cast_from(0.79012345679012345679e-2_f64) * t40105 + F::cast_from(0.17777777777777777778e-1_f64) * t40107 - F::cast_from(0.28793333333333333333e0_f64) * t40163 - F::new(0.24e0) * t25 * t606 * t47348 + F::cast_from(0.53333333333333333332e-1_f64) * t25 * t606 * t47438 + F::cast_from(0.79999999999999999998e-1_f64) * t25 * t1856 * t47442 - F::cast_from(0.88888888888888888888e-2_f64) * t25 * t1856 * t47446 - F::cast_from(0.17777777777777777778e-1_f64) * t25 * t5264 * t47450 + t17728 + F::cast_from(0.95977777777777777777e-1_f64) * t40213 + F::new(0.16e0) * t25 * t606 * t47414;
    (t47430, t47438, t47442, t47446, t47450, t47458)
}
