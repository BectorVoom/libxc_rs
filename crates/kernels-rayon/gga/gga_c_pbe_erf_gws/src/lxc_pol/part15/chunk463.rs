//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 15 (v4rho3sigma_3) CSE chunk 463/1352 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part15_v4rho3sigma_3_chunk463(t1759: f64, t1856: f64, t1765: f64, t606: f64, t1769: f64, t1756: f64, t1761: f64, t1767: f64, t1771: f64, t1844: f64, t1851: f64, t1852: f64, t25: f64) -> (f64, f64, f64, f64) {
    let t1857 = t1856 * t1759;
    let t1860 = t606 * t1765;
    let t1863 = t606 * t1769;
    let t1866 = t1844 + 0.23994444444444444444e-1_f64 * t1756 - 0.23994444444444444445e-1_f64 * t1761 + 0.71983333333333333334e-1_f64 * t1767 - 0.35991666666666666667e-1_f64 * t1771 + t1851 + 0.8888888888888888889e-2_f64 * t1852 - 0.22222222222222222222e-2_f64 * t25 * t1857 + 0.13333333333333333333e-1_f64 * t25 * t1860 - 0.66666666666666666667e-2_f64 * t25 * t1863;
    (t1857, t1860, t1863, t1866)
}
