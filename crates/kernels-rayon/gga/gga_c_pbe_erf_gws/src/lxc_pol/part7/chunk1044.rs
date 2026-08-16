//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1044/1242 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk1044(t1327: f64, t1333: f64, t40: f64, t460: f64, t4778: f64, t1423: f64, t1319: f64, t1322: f64, t18563: f64, t470: f64, t18639: f64, t456: f64, t4605: f64) -> (f64, f64, f64, f64, f64) {
    let t18941 = 120.0_f64 * t1333 * t1327;
    let t18943 = t40 * t4778 * t460;
    let t18944 = 4.0_f64 * t18943;
    let t18945 = t1333 * t1423;
    let t18946 = 120.0_f64 * t18945;
    let t18950 = 0.51947267698127589897e2_f64 * t470 * t1319 * t18563 * t1322;
    let t18954 = 0.1403573615389248977e2_f64 * t470 * t4605 * t18639 * t456;
    (t18941, t18944, t18946, t18950, t18954)
}
