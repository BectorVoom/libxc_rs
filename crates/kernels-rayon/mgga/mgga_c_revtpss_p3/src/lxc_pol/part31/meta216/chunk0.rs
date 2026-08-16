//! MGGA_C_REVTPSS lxc pol — lxc_pol part 31 (v4rho3sigma_6) CSE chunk 972/2259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk972(t5674: f64, t5675: f64, t5673: f64, t1388: f64, t1410: f64, t3931: f64, t3956: f64, t4022: f64, t4064: f64, t5606: f64, t5611: f64, t5614: f64, t5619: f64, t5623: f64, t5625: f64, t5629: f64, t5661: f64, t5666: f64, t5671: f64) -> (f64, f64) {
    let t5676 = t5674 * t5675;
    let t5677 = t5673 * t5676;
    let t5680 = t3956 + 0.40015750243531754507e-2_f64 * t5606 + 0.71456696863449561619e-5_f64 * t5611 - 0.85748036236139473944e-3_f64 * t1410 * t5614 - t4064 + 0.28582678745379824648e-4_f64 * t5619 - 0.50820002809285328225e-4_f64 * t5623 + 0.10003937560882938627e-2_f64 * t5625 + 0.42874018118069736972e-2_f64 * t1410 * t5629 - 0.21437009059034868486e-3_f64 * t1388 * t5661 - 0.12705000702321332056e-4_f64 * t5666 + 0.10003937560882938627e-2_f64 * t3931 - 0.12705000702321332056e-4_f64 * t4022 + 0.42874018118069736972e-3_f64 * t5671 * t5677;
    (t5677, t5680)
}
