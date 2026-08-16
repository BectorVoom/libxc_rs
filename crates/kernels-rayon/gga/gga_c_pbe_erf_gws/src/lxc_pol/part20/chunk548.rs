//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 20 (v4rho3sigma_8) CSE chunk 548/1389 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part20_v4rho3sigma_8_chunk548(t173: f64, t2824: f64, t184: f64, t199: f64, t1902: f64, t1905: f64, t1911: f64, t1915: f64, t1920: f64, t1926: f64, t1928: f64, t2755: f64, t2758: f64, t2788: f64, t2792: f64, t2794: f64, t2795: f64, t2798: f64, t2802: f64, t2806: f64, t2808: f64, t2818: f64) -> (f64, f64, f64, f64) {
    let t2825 = t173 * t2824;
    let t2826 = t2825 * t184;
    let t2828 = 2.0_f64 / 15.0_f64 * t2826 * t199;
    let t2829 = t2755 - t2758 - t2788 + t2792 - t2794 + t2795 + t2798 + t2802 + t1902 - t1905 + t1911 / 3.0_f64 + 0.60777777777777777777e-1_f64 * t1915 + t1920 + t1926 + t1928 - t2806 + t2808 + t2818 + t2828;
    (t2825, t2826, t2828, t2829)
}
