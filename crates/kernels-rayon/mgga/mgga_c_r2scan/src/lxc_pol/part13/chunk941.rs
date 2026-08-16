//! MGGA_C_R2SCAN lxc pol — lxc_pol part 13 (v4rho3sigma_3) CSE chunk 941/1253 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part13_v4rho3sigma_3_chunk941(t1060: f64, t1234: f64, t1058: f64, t2207: f64, t1607: f64, t3281: f64, t2124: f64, t5115: f64, t3295: f64, t10753: f64, t10759: f64, t10762: f64, t10765: f64, t10770: f64, t10774: f64, t10778: f64, t10782: f64, t10786: f64, t10790: f64, t10794: f64, t10797: f64) -> (f64, f64, f64, f64) {
    let t10799 = t1060 * t1234;
    let t10801 = t2207 * t1058 * t10799;
    let t10803 = t3281 * t1607;
    let t10804 = 0.10975748638225852664e-1_f64 * t10803;
    let t10805 = t2124 * t5115;
    let t10806 = t3295 * t10805;
    let t10808 = 0.86682217400542685632e-1_f64 * t10753 + t10759 - 0.43663693315433241792e-2_f64 * t10762 - 0.13099107994629972538e-1_f64 * t10765 + 0.47609969197673950972e-2_f64 * t10770 + 0.2600466522016280569e0_f64 * t10774 + 0.86682217400542685632e-1_f64 * t10778 + 0.10975748638225852664e0_f64 * t10782 - 0.43663693315433241792e-2_f64 * t10786 - 0.26198215989259945075e-1_f64 * t10790 + 0.21831846657716620896e-2_f64 * t10794 + 0.13099107994629972538e-1_f64 * t10797 + 0.65495539973149862688e-2_f64 * t10801 + t10804 - 0.27439371595564631661e-1_f64 * t10806;
    (t10799, t10803, t10805, t10808)
}
