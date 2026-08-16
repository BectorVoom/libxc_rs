//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2957/3317 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2957(t23452: f64, t974: f64, t981: f64, t15258: f64, t19467: f64, t4708: f64, t6226: f64, t19049: f64, t4734: f64, t1699: f64, t5023: f64, t68207: f64, t77657: f64, t78417: f64, t78422: f64, t78426: f64, t78428: f64, t78432: f64) -> (f64, f64, f64, f64, f64) {
    let t78435 = 0.14035736694323150897e2_f64 * t981 * t23452 * t974;
    let t78438 = 0.51947577317044391277e2_f64 * t981 * t19467 * t15258;
    let t78441 = 0.10526802520742363173e2_f64 * t981 * t6226 * t4708;
    let t78443 = 0.51947577317044391276e2_f64 * t19049 * t4734;
    let t78444 = -3.0_f64 * t1699 * t5023 * t68207 + t77657 - t78417 + t78422 - t78426 - t78428 - t78432 + t78435 - t78438 - t78441 - t78443;
    (t78435, t78438, t78441, t78443, t78444)
}
