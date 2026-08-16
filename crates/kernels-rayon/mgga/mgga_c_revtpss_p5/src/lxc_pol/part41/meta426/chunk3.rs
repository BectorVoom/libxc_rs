//! MGGA_C_REVTPSS lxc pol — lxc_pol part 41 (v4rho3tau_4) CSE chunk 1489/1497 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk1489(t31027: f64, t31545: f64, t31032: f64, t31548: f64, t31551: f64, t31542: f64, t1513: f64, t2: f64, t105872: f64, t105875: f64, t116919: f64, t116927: f64, t116930: f64, t116942: f64, t116968: f64, t116969: f64, t117499: f64, t117500: f64, t117505: f64, t117544: f64, t1504: f64, t21839: f64, t2349: f64, t31035: f64, t31039: f64, t31054: f64, t31058: f64, t31276: f64, t31283: f64, t31287: f64, t31541: f64, t4287: f64, t5823: f64, t5891: f64, t5895: f64, t5915: f64, t658: f64, t8258: f64, t8259: f64, t8267: f64, t8268: f64) -> f64 {
    let t118354 = t31027 * t31545;
    let t118359 = t31032 * t31548;
    let t118364 = t31032 * t31551;
    let t118369 = t31027 * t31542;
    let t118374 = t1513 * t2;
    let t118405 = -2.0_f64 / 3.0_f64 * t118354 - 5.0_f64 / 12.0_f64 * t8258 * t31039 * t5915 + 10.0_f64 / 27.0_f64 * t118359 + 25.0_f64 / 108.0_f64 * t8267 * t116942 * t5895 + 5.0_f64 / 9.0_f64 * t118364 + 25.0_f64 / 72.0_f64 * t8267 * t31054 * t5823 - 20.0_f64 / 9.0_f64 * t118369 - 25.0_f64 / 18.0_f64 * t8258 * t31054 * t31541 + 5.0_f64 / 6.0_f64 * t117544 * t8268 * t118374 - 5.0_f64 / 18.0_f64 * t31287 * t31058 * t21839 + 3.0_f64 * t116919 * t8259 * t105872 - 5.0_f64 / 4.0_f64 * t31035 * t8268 * t5891 * t658 - 3.0_f64 / 2.0_f64 * t31035 * t8259 * t105875 + 5.0_f64 / 6.0_f64 * t8258 * t8268 * t4287 * t1504 + 22.0_f64 / 9.0_f64 * t116927 - 55.0_f64 / 27.0_f64 * t116930 + t116968 + 55.0_f64 / 27.0_f64 * t116969 - 5.0_f64 / 2.0_f64 * t117499 * t117500 * t31276 + 5.0_f64 / 9.0_f64 * t117505 * t2349 * t1513 * t31283;
    t118405
}
