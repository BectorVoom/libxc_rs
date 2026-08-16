//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 19 (v4rho3sigma_7) CSE chunk 1059/1404 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part19_v4rho3sigma_7_chunk1059(t11889: f64, t2210: f64, t858: f64, t884: f64, t904: f64, t933: f64, t9807: f64, t11874: f64, t11876: f64, t11880: f64, t11885: f64, t11888: f64, t6506: f64, t9041: f64, t9086: f64, t9096: f64, t929: f64, t9549: f64, t9565: f64) -> (f64, f64, f64) {
    let t11891 = t2210 * t858 * t11889;
    let t11893 = t884 * t11891 / 8.0_f64;
    let t11896 = t933 * t904 * t9807;
    let t11899 = t9549 - t11874 + t11876 - t9041 + t11880 + t11885 - t11888 + t11893 - 119.0_f64 / 3456.0_f64 * t6506 + t9086 - t9096 - t929 * t11896 / 768.0_f64 - t9565;
    (t11893, t11896, t11899)
}
