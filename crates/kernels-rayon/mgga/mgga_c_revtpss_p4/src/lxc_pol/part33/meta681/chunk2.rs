//! MGGA_C_REVTPSS lxc pol — lxc_pol part 33 (v4rho3sigma_8) CSE chunk 2223/2275 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2223(t5: f64, t111468: f64, t111493: f64, t111521: f64, t111548: f64, t111577: f64, t111623: f64, t111652: f64, t111680: f64, t117: f64, t105859: f64, t105863: f64, t105889: f64, t105894: f64, t105897: f64, t108067: f64, t108068: f64, t108076: f64, t1310: f64, t13426: f64, t18227: f64, t18245: f64, t21891: f64, t27060: f64, t29432: f64, t29444: f64, t30716: f64, t34446: f64, t4248: f64, t4293: f64, t508: f64, t5787: f64, t5887: f64, t7586: f64, t7591: f64, t8158: f64, t8237: f64) -> (f64, f64) {
    let t7 = piecewise3(0.0_f64 < t5, t5, -t5);
    let t8 = -t7 <= -0.999999999999e0_f64;
    let t111684 = piecewise3(t8, 0.0_f64, t111468 + t111493 + t111521 + t111548 + t111577 + t111623 + t111652 + t111680);
    let t111685 = t111684 * t117;
    let t111690 = -t111685 * t508 - t1310 * t30716 - 4.0_f64 * t13426 * t8158 - 4.0_f64 * t18227 * t8158 - 2.0_f64 * t18245 * t7591 - 4.0_f64 * t21891 * t7586 - 4.0_f64 * t27060 * t5887 - 4.0_f64 * t29432 * t5887 - 4.0_f64 * t29444 * t4248 - 4.0_f64 * t34446 * t4293 + 2.0_f64 * t5787 * t8237 - t105859 - t105863 - t105889 + t105894 + t105897 + t108067 + t108068 - t108076;
    (t111685, t111690)
}
