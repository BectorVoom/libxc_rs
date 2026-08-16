//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1046/1242 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk1046(t1327: f64, t1438: f64, t461: f64, t4862: f64, t1326: f64, t1422: f64, t40: f64, t1322: f64, t18639: f64, t470: f64, t4734: f64, t1336: f64) -> (f64, f64, f64, f64, f64) {
    let t18958 = t1438 * t1327;
    let t18959 = 192.0_f64 * t18958;
    let t18961 = 480.0_f64 * t4862 * t461;
    let t18963 = t40 * t1422 * t1326;
    let t18964 = 6.0_f64 * t18963;
    let t18968 = 0.6233672123775310788e3_f64 * t470 * t4734 * t18639 * t1322;
    let t18969 = t1336 * t1327;
    (t18959, t18961, t18964, t18968, t18969)
}
