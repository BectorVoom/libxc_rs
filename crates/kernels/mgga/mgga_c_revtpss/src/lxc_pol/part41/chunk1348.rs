//! MGGA_C_REVTPSS lxc pol — lxc_pol part 41 (v4rho3tau_4) CSE chunk 1348/1356 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk1348<F: Float>(t31027: F, t31545: F, t31032: F, t31548: F, t31551: F, t31542: F, t1513: F, t2: F, t105872: F, t105875: F, t116919: F, t116927: F, t116930: F, t116942: F, t116968: F, t116969: F, t117499: F, t117500: F, t117505: F, t117544: F, t1504: F, t21839: F, t2349: F, t31035: F, t31039: F, t31054: F, t31058: F, t31276: F, t31283: F, t31287: F, t31541: F, t4287: F, t5823: F, t5891: F, t5895: F, t5915: F, t658: F, t8258: F, t8259: F, t8267: F, t8268: F) -> (F,) {
    let t118354 = t31027 * t31545;
    let t118359 = t31032 * t31548;
    let t118364 = t31032 * t31551;
    let t118369 = t31027 * t31542;
    let t118374 = t1513 * t2;
    let t118405 = -2.0 / 3.0 * t118354 - 5.0 / 12.0 * t8258 * t31039 * t5915 + 10.0 / 27.0 * t118359 + 25.0 / 108.0 * t8267 * t116942 * t5895 + 5.0 / 9.0 * t118364 + 25.0 / 72.0 * t8267 * t31054 * t5823 - 20.0 / 9.0 * t118369 - 25.0 / 18.0 * t8258 * t31054 * t31541 + 5.0 / 6.0 * t117544 * t8268 * t118374 - 5.0 / 18.0 * t31287 * t31058 * t21839 + 3.0 * t116919 * t8259 * t105872 - 5.0 / 4.0 * t31035 * t8268 * t5891 * t658 - 3.0 / 2.0 * t31035 * t8259 * t105875 + 5.0 / 6.0 * t8258 * t8268 * t4287 * t1504 + 22.0 / 9.0 * t116927 - 55.0 / 27.0 * t116930 + t116968 + 55.0 / 27.0 * t116969 - 5.0 / 2.0 * t117499 * t117500 * t31276 + 5.0 / 9.0 * t117505 * t2349 * t1513 * t31283;
    (t118405,)
}
