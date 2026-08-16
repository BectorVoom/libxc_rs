//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 1191/1222 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk1191(t871: f64, t89818: f64, t89861: f64, t90322: f64, t90478: f64, t10214: f64, t1526: f64, t15567: f64, t18961: f64, t18968: f64, t21181: f64, t21196: f64, t21204: f64, t21933: f64, t21949: f64, t22161: f64, t231: f64, t2320: f64, t342: f64, t343: f64, t3806: f64, t44674: f64, t72977: f64, t82494: f64, t82497: f64, t82552: f64) -> (f64, f64) {
    let t90481 = t871 * (t89818 + t89861 + t90322 + t90478);
    let t90516 = -t82552 / 4.0_f64 + 2.0_f64 * t21933 - t1526 * t2320 * t10214 * t21181 / 2.0_f64 + t15567 * t18968 * t21204 / 2.0_f64 + t1526 * t2320 * t21949 / 2.0_f64 + 2.0_f64 / 3.0_f64 * t1526 * t3806 * t44674 * t21181 - t15567 * t18961 * t21196 / 3.0_f64 - t82494 / 12.0_f64 + t82497 / 6.0_f64 - t342 * t343 * t231 * t22161 / 4.0_f64 + t72977 / 6.0_f64;
    (t90481, t90516)
}
