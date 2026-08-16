//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 729/1242 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk729(t164: f64, t5984: f64, t1964: f64, t528: f64, t163: f64, t169: f64, t171: f64, t4563: f64, t5891: f64, t5895: f64, t5898: f64, t5962: f64, t5969: f64, t5973: f64, t5977: f64, t5980: f64, t5982: f64) -> (f64, f64) {
    let t5985 = t5984 * t164;
    let t5986 = 0.1186530987165140469e-3_f64 * t5985;
    let t5988 = 0.94516221669423353502e-1_f64 * t528 * t1964;
    let t5989 = -0.14862827083471493416e-2_f64 * t5891 - t5895 - t5898 - 0.53884053046145740922e-2_f64 * t169 * t171 * t5962 * t163 - 0.71845404061527654564e-1_f64 * t5969 + 0.26942026523072870461e-1_f64 * t5973 - t5977 - 0.31505407223141117834e-1_f64 * t4563 * t164 - 0.94516221669423353502e-1_f64 * t5980 - 0.94516221669423353502e-1_f64 * t5982 - t5986 + t5988;
    (t5985, t5989)
}
