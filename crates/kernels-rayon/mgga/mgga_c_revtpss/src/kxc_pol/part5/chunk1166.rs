//! MGGA_C_REVTPSS kxc pol — kxc_pol part 5 (v3rho3_2) CSE chunk 1166/1422 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_kxc_pol_part5_v3rho3_2_chunk1166(t18534: f64, t18553: f64, t18568: f64, t18583: f64, t225: f64, t1553: f64, t73: f64, t2475: f64, t5966: f64, t775: f64, t4343: f64, t4416: f64) -> (f64, f64, f64, f64) {
    let t18586 = (t18534 + t18553 + t18568 + t18583) * t225;
    let t18592 = t1553 * t73;
    let t18599 = t2475 * t5966;
    let t18600 = t18599 * t775;
    let t18603 = t4416 * t4343;
    (t18586, t18592, t18600, t18603)
}
