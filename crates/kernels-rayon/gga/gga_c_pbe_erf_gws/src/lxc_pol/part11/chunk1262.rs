//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1262/1302 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk1262(t11630: f64, t11782: f64, t11668: f64, t13243: f64, t1105: f64, t13489: f64, t2147: f64, t3116: f64, t337: f64, t3854: f64, t6241: f64, t11478: f64, t3139: f64, t8903: f64) -> (f64, f64, f64, f64, f64) {
    let t50049 = t11782 * t11630 / 16.0_f64;
    let t50051 = t11668 * t13243 / 6.0_f64;
    let t50056 = t3116 * t2147 * t337 * t13489 * t1105 / 12.0_f64;
    let t50069 = t6241 * t3854;
    let t50073 = 3.0_f64 / 8.0_f64 * t8903 * t3139 * t11478 * t50069;
    (t50049, t50051, t50056, t50069, t50073)
}
