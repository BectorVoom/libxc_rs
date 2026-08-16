//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1129/1302 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk1129<F: Float>(t17037: F, t47377: F, t11: F, t5089: F, t1714: F, t17957: F, t17983: F, t25: F, t25049: F, t41974: F, t41976: F, t47929: F, t47940: F, t47944: F, t47949: F, t47953: F, t47957: F, t5061: F, t657: F) -> (F, F) {
    let t48017 = t17037 * t47377;
    let t48034 = t11 * t5089 * t48017;
    let t48037 = -F::cast_from(0.35555555555555555556e-1_f64) * t41974 + F::cast_from(0.10666666666666666667e0_f64) * t41976 - F::cast_from(0.69135802469135802468e-2_f64) * t25 * t17957 * t47957 - F::cast_from(0.66666666666666666667e-2_f64) * t25 * t657 * t47929 + F::cast_from(0.35555555555555555554e-1_f64) * t25 * t5061 * t48017 - F::cast_from(0.79999999999999999998e-1_f64) * t25 * t1714 * t47940 - F::cast_from(0.66666666666666666666e-2_f64) * t25 * t1714 * t47944 + F::cast_from(0.16e0_f64) * t25 * t657 * t47949 + F::cast_from(0.39999999999999999999e-1_f64) * t25 * t657 * t47953 + t17983 + F::cast_from(0.4798888888888888889e0_f64) * t48034 + F::cast_from(0.14929876543209876543e0_f64) * t25049;
    (t48034, t48037)
}
