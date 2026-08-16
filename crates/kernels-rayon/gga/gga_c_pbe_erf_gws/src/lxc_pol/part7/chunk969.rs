//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 969/1242 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk969(t401: f64, t5025: f64, t2718: f64, t658: f64, t1251: f64, t1721: f64, t1715: f64, t5065: f64, t1697: f64, t191: f64, t205: f64, t16974: f64, t16979: f64, t16995: f64, t17005: f64, t17022: f64, t17038: f64, t1714: f64, t25: f64, t5061: f64, t657: f64) -> f64 {
    let t17939 = t401 * t5025;
    let t17944 = t2718 * t658;
    let t17949 = t1251 * t1721;
    let t17951 = t1251 * t1715;
    let t17953 = t401 * t5065;
    let t17957 = t191 / t205 / t1697;
    let t17964 = -0.79999999999999999998e-1_f64 * t25 * t1714 * t16995 - 0.66666666666666666666e-2_f64 * t25 * t1714 * t17005 - 0.35555555555555555556e-1_f64 * t17939 + 0.35555555555555555554e-1_f64 * t25 * t5061 * t17038 + 0.79012345679012345678e-1_f64 * t17944 - 0.66666666666666666667e-2_f64 * t25 * t657 * t16979 - 0.44444444444444444445e-1_f64 * t17949 - 0.14814814814814814815e-1_f64 * t17951 + 0.79012345679012345679e-2_f64 * t17953 - 0.69135802469135802468e-2_f64 * t25 * t17957 * t16974 - 0.24e0_f64 * t25 * t657 * t17022;
    t17964
}
