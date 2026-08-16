//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1087/1302 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk1087(t11: f64, t47401: f64, t571: f64, t12345: f64, t2554: f64, t10777: f64, t3346: f64, t2560: f64, t10783: f64, t17728: f64, t1856: f64, t25: f64, t40105: f64, t40107: f64, t40163: f64, t40213: f64, t47348: f64, t47414: f64, t5264: f64, t606: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t47430 = t11 * t571 * t47401;
    let t47438 = t2554 * t12345;
    let t47442 = t10777 * t3346;
    let t47446 = t2560 * t12345;
    let t47450 = t10783 * t3346;
    let t47458 = -0.35991666666666666667e-1_f64 * t47430 + 0.79012345679012345679e-2_f64 * t40105 + 0.17777777777777777778e-1_f64 * t40107 - 0.28793333333333333333e0_f64 * t40163 - 0.24e0_f64 * t25 * t606 * t47348 + 0.53333333333333333332e-1_f64 * t25 * t606 * t47438 + 0.79999999999999999998e-1_f64 * t25 * t1856 * t47442 - 0.88888888888888888888e-2_f64 * t25 * t1856 * t47446 - 0.17777777777777777778e-1_f64 * t25 * t5264 * t47450 + t17728 + 0.95977777777777777777e-1_f64 * t40213 + 0.16e0_f64 * t25 * t606 * t47414;
    (t47430, t47438, t47442, t47446, t47450, t47458)
}
