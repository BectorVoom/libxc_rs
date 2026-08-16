//! MGGA_C_REVTPSS lxc pol — lxc_pol part 55 (v4rho2sigma2_10) CSE chunk 1252/1306 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part55_v4rho2sigma2_10_chunk1252(t125609: f64, t8477: f64, t1955: f64, t32715: f64, t128617: f64, t1426: f64, t7286: f64, t786: f64, t122391: f64, t122393: f64, t122399: f64, t125717: f64, t125721: f64, t125732: f64, t128618: f64, t26304: f64, t27840: f64, t27869: f64, t28012: f64, t32720: f64, t46361: f64, t7304: f64, t8702: f64, t8705: f64) -> (f64, f64, f64) {
    let t128691 = t8477 * t125609;
    let t128694 = t1955 * t32715;
    let t128709 = t128617 * t1426;
    let t128711 = t786 * t128709 * t7286;
    let t128713 = 0.8673628188205199462e0_f64 * t128618 * t7304 - 0.11423947533020470523e1_f64 * t128691 * t32720 + 0.8673628188205199462e0_f64 * t128694 * t27869 + 0.6854368519812282314e1_f64 * t8477 * t8705 * t46361 * t26304 * t27840 - 0.22312397525430606492e-2_f64 * t125717 - 0.7437465841810202164e-2_f64 * t125721 - 0.69416347856895220197e-2_f64 * t125732 - 0.8673628188205199462e0_f64 * t8702 * t28012 + 0.14456046980341999104e-1_f64 * t122391 - 0.25702851531048074406e-1_f64 * t122393 + 0.14456046980341999104e-1_f64 * t128711 + t122399;
    (t128694, t128709, t128713)
}
