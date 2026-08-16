//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 883/1242 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk883(t1783: f64, t1795: f64, t5113: f64, t633: f64, t4897: f64, t661: f64, t7115: f64, t7116: f64, t4972: f64, t5218: f64, t562: f64, t7148: f64) -> (f64, f64, f64, f64) {
    let t16832 = 16.0_f64 / 5.0_f64 * t1783 * t1795;
    let t16834 = 16.0_f64 / 5.0_f64 * t633 * t5113;
    let t16838 = 32.0_f64 / 15.0_f64 * t7115 * t7116 * t661 * t4897;
    let t16842 = 64.0_f64 / 15.0_f64 * t5218 * t7148 * t562 * t4972;
    (t16832, t16834, t16838, t16842)
}
