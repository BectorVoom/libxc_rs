//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 921/1302 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk921(t18639: f64, t456: f64, t4605: f64, t470: f64, t16576: f64, t88: f64, t1327: f64, t1438: f64, t461: f64, t4862: f64, t1322: f64, t4734: f64) -> (f64, f64, f64, f64, f64) {
    let t18954 = 0.1403573615389248977e2_f64 * t470 * t4605 * t18639 * t456;
    let t18955 = t16576 * t88;
    let t18956 = 384.0_f64 * t18955;
    let t18958 = t1438 * t1327;
    let t18959 = 192.0_f64 * t18958;
    let t18961 = 480.0_f64 * t4862 * t461;
    let t18968 = 0.6233672123775310788e3_f64 * t470 * t4734 * t18639 * t1322;
    (t18954, t18956, t18959, t18961, t18968)
}
