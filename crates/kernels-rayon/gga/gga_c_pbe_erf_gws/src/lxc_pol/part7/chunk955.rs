//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 955/1242 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk955(t1251: f64, t1860: f64, t401: f64, t5233: f64, t2718: f64, t607: f64, t16670: f64, t16680: f64, t16688: f64, t16695: f64, t16732: f64, t16734: f64, t16736: f64, t16743: f64, t16749: f64, t1856: f64, t25: f64, t606: f64) -> f64 {
    let t17695 = t1251 * t1860;
    let t17700 = t401 * t5233;
    let t17711 = t2718 * t607;
    let t17713 = -0.63985185185185185184e-1_f64 * t16732 + 0.53320987654320987654e-1_f64 * t16734 + 0.47988888888888888888e-1_f64 * t16736 - 0.10664197530864197531e0_f64 * t16743 - 0.35991666666666666667e-1_f64 * t16749 + 0.88888888888888888889e-1_f64 * t17695 - 0.66666666666666666666e-2_f64 * t25 * t1856 * t16680 + 0.10666666666666666667e0_f64 * t17700 + 0.16e0_f64 * t25 * t606 * t16688 + 0.39999999999999999999e-1_f64 * t25 * t606 * t16695 - 0.79999999999999999998e-1_f64 * t25 * t1856 * t16670 + 0.79012345679012345678e-1_f64 * t17711;
    t17713
}
