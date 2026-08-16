//! MGGA_C_REVTPSS lxc pol — lxc_pol part 27 (v4rho3sigma_2) CSE chunk 1297/1333 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part27_v4rho3sigma_2_chunk1297(t2122: f64, t92569: f64, t25163: f64, t7575: f64, t92576: f64, t92584: f64, t45958: f64, t7565: f64, t10301: f64, t26754: f64, t1923: f64, t2123: f64, t25146: f64, t25162: f64, t26792: f64, t26795: f64, t6960: f64, t92565: f64, t92568: f64, t92581: f64, t92588: f64, t92639: f64, t92696: f64) -> f64 {
    let t96752 = t2122 * t92569;
    let t96757 = t7575 * t25163;
    let t96760 = t2122 * t92576;
    let t96765 = t2122 * t92584;
    let t96773 = t45958 * t7565;
    let t96776 = t10301 * t26754;
    let t96779 = -t1923 * t7575 * t25146 / 2.0_f64 + 30.0_f64 * t92568 * t96752 - 10.0_f64 * t92565 * t26795 - 10.0_f64 * t25162 * t96757 - 10.0_f64 * t25162 * t96760 - 15.0_f64 * t26792 * t92581 - 5.0_f64 * t25162 * t96765 - 5.0_f64 * t92588 * t26795 + t92639 * t2123 - 15.0_f64 * t26792 * t92696 + 5.0_f64 / 2.0_f64 * t96773 * t6960 + 5.0_f64 * t96776 * t6960;
    t96779
}
