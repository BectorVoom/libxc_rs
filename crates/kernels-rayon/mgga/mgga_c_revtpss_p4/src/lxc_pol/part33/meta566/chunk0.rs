//! MGGA_C_REVTPSS lxc pol — lxc_pol part 33 (v4rho3sigma_8) CSE chunk 1967/2275 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1967(t1785: f64, t8184: f64, t2137: f64, t6593: f64, t467: f64, t1782: f64, t1791: f64, t1797: f64, t26824: f64, t26870: f64, t26877: f64, t29010: f64, t29062: f64, t29072: f64, t29077: f64, t29086: f64, t29089: f64, t484: f64, t6611: f64, t6647: f64, t6653: f64, t6659: f64, t6663: f64, t6673: f64, t6683: f64, t6690: f64, t7607: f64, t7613: f64, t7624: f64) -> (f64, f64, f64, f64) {
    let t30812 = t1785 * t8184;
    let t30815 = t2137 * t6593;
    let t30816 = t467 * t30815;
    let t30839 = 0.47637797908966374413e-3_f64 * t7624 * t6673 + 0.57165357490759649296e-3_f64 * t29072 - 0.30488190661738479624e-2_f64 * t29077 + 0.85748036236139473944e-3_f64 * t29010 * t1797 - 0.45732285992607719436e-2_f64 * t30812 * t484 + 0.14481890564325777821e-1_f64 * t30816 * t484 - t26877 - 0.57165357490759649296e-3_f64 * t7624 * t6683 - 0.85748036236139473944e-3_f64 * t26870 * t6690 - 0.85748036236139473944e-3_f64 * t29086 * t1791 + 0.85748036236139473944e-3_f64 * t26824 * t6611 + 0.45732285992607719436e-2_f64 * t29062 * t1791 - 0.42874018118069736972e-3_f64 * t7613 * t6647 + t7607 * t6653 / 216.0_f64 + t29089 * t1782 / 54.0_f64 - t7607 * t6659 / 288.0_f64 - t7607 * t6663 / 144.0_f64;
    (t30812, t30815, t30816, t30839)
}
