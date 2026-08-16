//! GGA_C_GAPLOC lxc pol — lxc_pol part 46 (v4rhosigma3_11) CSE chunk 928/1029 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part46_v4rhosigma3_11_chunk928(t123: f64, t1841: f64, t1843: f64, t42921: f64, t42925: f64, t42931: f64, t42934: f64, t42937: f64, t42940: f64, t42943: f64, t42948: f64, t42951: f64, t42954: f64, t42956: f64, t42961: f64, t42964: f64, t42968: f64, t42971: f64, t42974: f64, t42978: f64, t734: f64) -> f64 {
    let t42979 = 0.85450291446024714263e-3_f64 * t1841 * t1843 * t42921 - 0.85450291446024714263e-3_f64 * t1841 * t42925 * t123 * t734 - 0.64087718584518535698e-3_f64 * t42931 - t42934 - t42937 - t42940 + t42943 + t42948 - 0.1922631557535556071e-2_f64 * t42951 - t42954 + 0.1281754371690370714e-2_f64 * t42956 - t42961 + t42964 - t42968 - t42971 - t42974 - t42978;
    t42979
}
