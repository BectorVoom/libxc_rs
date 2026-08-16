//! MGGA_C_REVTPSS lxc pol — lxc_pol part 20 (v4rho4_0) CSE chunk 1041/1798 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1041(t10988: f64, t689: f64, t2444: f64, t887: f64, t252: f64, t2769: f64, t786: f64, t2771: f64, t676: f64, t123: f64, t2435: f64, t2448: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t10989 = t689 * t10988;
    let t10991 = t2444 * t887;
    let t10992 = t689 * t10991;
    let t10994 = t252 * t2769;
    let t10995 = t786 * t10994;
    let t10996 = t676 * t2771;
    let t10997 = t123 * t10996;
    let t10998 = t10995 * t10997;
    let t11000 = t2435 * t2448;
    (t10989, t10991, t10992, t10994, t10995, t10997, t10998, t11000)
}
