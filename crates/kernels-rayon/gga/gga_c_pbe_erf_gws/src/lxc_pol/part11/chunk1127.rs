//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1127/1302 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk1127(t47377: f64, t5002: f64, t11: f64, t625: f64, t1697: f64, t47733: f64, t16972: f64, t16970: f64, t10523: f64, t3354: f64, t1714: f64, t24980: f64, t25: f64, t32405: f64, t40954: f64, t40956: f64, t40958: f64, t40960: f64, t41939: f64, t41941: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t47949 = t5002 * t47377;
    let t47951 = t11 * t625 * t47949;
    let t47953 = t1697 * t47733;
    let t47955 = t11 * t625 * t47953;
    let t47957 = t16972 * t47377;
    let t47959 = t11 * t16970 * t47957;
    let t47969 = t10523 * t3354;
    let t47973 = 0.8638e0_f64 * t47951 + 0.21595e0_f64 * t47955 - 0.10664197530864197531e0_f64 * t47959 + 0.47988888888888888888e-1_f64 * t40954 + 0.53320987654320987654e-1_f64 * t40956 - 0.19195555555555555555e0_f64 * t40958 + 0.28793333333333333333e0_f64 * t40960 + 0.79012345679012345678e-1_f64 * t24980 + 0.88888888888888888889e-1_f64 * t32405 + 0.79012345679012345679e-2_f64 * t41939 + 0.17777777777777777778e-1_f64 * t41941 + 0.79999999999999999998e-1_f64 * t25 * t1714 * t47969;
    (t47949, t47951, t47953, t47955, t47957, t47959, t47969, t47973)
}
