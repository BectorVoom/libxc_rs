//! MGGA_C_REVTPSS lxc pol — lxc_pol part 33 (v4rho3sigma_8) CSE chunk 1992/2275 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1992(t159: f64, t8779: f64, t218: f64, t816: f64, t10685: f64, t1946: f64, t10671: f64, t7033: f64, t25255: f64, t2689: f64, t10690: f64, t1945: f64, t9646: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t92993 = t8779 * t159;
    let t92995 = t92993 * t218 * t816;
    let t92996 = 455.0_f64 / 1296.0_f64 * t92995;
    let t92997 = t1946 * t10685;
    let t92998 = 0.7558530601555998074e-1_f64 * t92997;
    let t92999 = t7033 * t10671;
    let t93000 = 0.25692334753583138159e-2_f64 * t92999;
    let t93001 = t2689 * t25255;
    let t93007 = t9646 * t1945 * t10690;
    (t92993, t92996, t92998, t93000, t93001, t93007)
}
