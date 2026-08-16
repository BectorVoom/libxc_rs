//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1129/1302 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk1129(t17037: f64, t47377: f64, t11: f64, t5089: f64, t1714: f64, t17957: f64, t17983: f64, t25: f64, t25049: f64, t41974: f64, t41976: f64, t47929: f64, t47940: f64, t47944: f64, t47949: f64, t47953: f64, t47957: f64, t5061: f64, t657: f64) -> (f64, f64) {
    let t48017 = t17037 * t47377;
    let t48034 = t11 * t5089 * t48017;
    let t48037 = -0.35555555555555555556e-1_f64 * t41974 + 0.10666666666666666667e0_f64 * t41976 - 0.69135802469135802468e-2_f64 * t25 * t17957 * t47957 - 0.66666666666666666667e-2_f64 * t25 * t657 * t47929 + 0.35555555555555555554e-1_f64 * t25 * t5061 * t48017 - 0.79999999999999999998e-1_f64 * t25 * t1714 * t47940 - 0.66666666666666666666e-2_f64 * t25 * t1714 * t47944 + 0.16e0_f64 * t25 * t657 * t47949 + 0.39999999999999999999e-1_f64 * t25 * t657 * t47953 + t17983 + 0.4798888888888888889e0_f64 * t48034 + 0.14929876543209876543e0_f64 * t25049;
    (t48034, t48037)
}
