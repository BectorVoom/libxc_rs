//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2855/3317 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2855(t23121: f64, t40188: f64, t40121: f64, t40132: f64, t40139: f64, t40088: f64, t40099: f64, t40103: f64, t40115: f64, t40131: f64, t40137: f64, t50048: f64, t76986: f64, t76987: f64, t76988: f64, t76991: f64, t76992: f64, t76995: f64) -> (f64, f64, f64, f64, f64) {
    let t76997 = 24.0_f64 * t40188 * t23121;
    let t76998 = 0.10389515463408878255e3_f64 * t40121;
    let t76999 = 0.5848223622634646207e0_f64 * t40132;
    let t77000 = 4.0_f64 * t40139;
    let t77001 = t76986 + t40088 - t76987 + t76988 + t40099 + t40103 + t76991 + t50048 + t76992 + t76995 + t76997 - t40115 + t76998 - t40131 - t76999 - t40137 + t77000;
    (t76997, t76998, t76999, t77000, t77001)
}
