//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 19 (v4rho3sigma_7) CSE chunk 350/1404 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part19_v4rho3sigma_7_chunk350(t247: f64, t991: f64, t251: f64, t108: f64, t726: f64, t728: f64, t950: f64, t954: f64, t1008: f64, t1012: f64, t1021: f64, t1026: f64, t1035: f64, t1039: f64, t1048: f64, t256: f64, t267: f64, t585: f64, t638: f64, t716: f64, t722: f64, t725: f64, t737: f64, t999: f64) -> (f64, f64, f64, f64) {
    let t1061 = t991 * t247;
    let t1062 = t1061 * t251;
    let t1069 = (4.0_f64 / 3.0_f64 * t726 * t950 + 4.0_f64 / 3.0_f64 * t728 * t954) * t108;
    let t1072 = t999 + t1008 + t585 + t1012 - t1021 + t1026 + t1035 + t638 + t1039 - t1048 + t1062 * t256 / 3.0_f64 + t716 + t722 + t725 - t1069 * t267 / 15.0_f64 - t737;
    (t1061, t1062, t1069, t1072)
}
